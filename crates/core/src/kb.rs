//! Knowledge base operations on top of the vault: asserting facts with
//! time-based contradiction handling, history, usage tracking and the
//! audit log. Facts are never deleted; contradicted facts get an end time.

use crate::doc::Item;
use crate::term::{Atom, Fact, GroundAtom, Rule, Term, Value};
use crate::time::Timestamp;
use crate::vault::{RuleStatus, Vault};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum AssertOutcome {
    /// A new fact was stored; `closed` lists facts it ended.
    Added { closed: Vec<Fact> },
    /// The same fact was already current; its confidence was raised.
    Confirmed { conf: f64 },
}

/// A rule ready for the reasoner.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedRule {
    /// `<file id>#<index>`.
    pub id: String,
    pub rule: Rule,
    pub conf: f64,
    pub status: RuleStatus,
    pub module: String,
}

/// One change, as written to the audit log.
#[derive(Debug, Clone, PartialEq)]
pub struct JournalEntry {
    pub at: Timestamp,
    pub action: String,
    pub subject: String,
    pub reason: String,
    /// The id of the fact added or closed, if any.
    pub fact: Option<String>,
}

pub struct KnowledgeBase {
    pub vault: Vault,
    /// Write changes to the log directory.
    pub logging: bool,
    /// Every change since the knowledge base was opened, in order (also when
    /// `logging` is off). Take a mark with [`KnowledgeBase::journal_len`] and
    /// read what happened since with [`KnowledgeBase::journal_since`].
    journal: RefCell<Vec<JournalEntry>>,
    /// Inside a transaction: log lines wait here until it commits.
    deferred: RefCell<Option<Vec<JournalEntry>>>,
    /// The next fact id number (`f<n>`) and event id number (`e<n>`).
    next_fact: u64,
    next_event: u64,
}

/// Does ground atom `g` match `pattern` (variables bind consistently)?
pub fn matches(pattern: &Atom, g: &GroundAtom) -> bool {
    if pattern.pred != g.pred || pattern.args.len() != g.args.len() {
        return false;
    }
    let mut b: HashMap<&str, &Value> = HashMap::new();
    for (t, v) in pattern.args.iter().zip(&g.args) {
        match t {
            Term::Val(c) => {
                if c != v {
                    return false;
                }
            }
            Term::Var(name) if name == "_" => {}
            Term::Var(name) => match b.get(name.as_str()) {
                Some(prev) if *prev != v => return false,
                Some(_) => {}
                None => {
                    b.insert(name, v);
                }
            },
        }
    }
    true
}

/// Raise confidence on confirmation: halve the remaining doubt.
pub fn confirm_conf(old: f64, new: f64) -> f64 {
    let base = old.max(new);
    let c = (base + (1.0 - base) * 0.5).min(1.0);
    (c * 1000.0).round() / 1000.0
}

impl KnowledgeBase {
    pub fn new(vault: Vault) -> Self {
        let mut kb = KnowledgeBase { vault, logging: true, journal: RefCell::new(Vec::new()), deferred: RefCell::new(None), next_fact: 1, next_event: 1 };
        kb.scan_ids();
        kb
    }

    /// Load a vault from disk, seeding the core ontology if it is missing.
    pub fn open(root: impl Into<PathBuf>) -> io::Result<Self> {
        let mut vault = Vault::load(root)?;
        // Seed every built-in core file the vault does not have yet (a vault
        // made by an older build gets the new layers too).
        let mut added = false;
        for (stem, rf) in crate::default_core() {
            if !vault.core.contains_key(&stem) {
                vault.core.insert(stem, rf);
                added = true;
            }
        }
        for (stem, rf) in crate::default_lexicon() {
            if !vault.lexicon.contains_key(&stem) {
                vault.lexicon.insert(stem, rf);
                added = true;
            }
        }
        if added {
            vault.save_all()?;
        }
        Ok(KnowledgeBase::new(vault))
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.vault.save()
    }

    /// Continue numbering after the highest fact and event ids in the vault.
    fn scan_ids(&mut self) {
        let num = |s: &str, p: char| s.strip_prefix(p).and_then(|n| n.parse::<u64>().ok());
        for (_, f) in self.vault.all_facts() {
            if let Some(n) = f.id.as_deref().and_then(|i| num(i, 'f')) {
                self.next_fact = self.next_fact.max(n + 1);
            }
            for v in &f.atom.args {
                if let Some(n) = v.as_sym().and_then(|s| num(s, 'e')) {
                    if f.atom.pred == "event" {
                        self.next_event = self.next_event.max(n + 1);
                    }
                }
            }
        }
    }

    /// A new fact id (`f12`).
    pub fn new_fact_id(&mut self) -> String {
        let id = format!("f{}", self.next_fact);
        self.next_fact += 1;
        id
    }

    /// A new event id (`e7`): statements with more than two participants become events.
    pub fn new_event_id(&mut self) -> String {
        let id = format!("e{}", self.next_event);
        self.next_event += 1;
        id
    }

    /// The stored fact with this id.
    pub fn fact_by_id(&self, id: &str) -> Option<Fact> {
        self.all_facts().into_iter().find(|f| f.id.as_deref() == Some(id))
    }

    /// End exactly the fact with this id (not other copies of its atom).
    pub fn close_by_id(&mut self, id: &str, at: Timestamp, reason: &str) -> Option<Fact> {
        let target = self.fact_by_id(id).filter(|f| f.is_open())?;
        self.vault.update_facts(|f| {
            if f.id.as_deref() == Some(id) && f.is_open() {
                f.until = Some(at);
                true
            } else {
                false
            }
        });
        self.log_fact(at, "close", &target.atom.to_string(), reason, Some(id));
        Some(Fact { until: Some(at), ..target })
    }

    pub fn reload(&mut self) -> io::Result<()> {
        self.vault.reload()?;
        for (stem, rf) in crate::default_core() {
            self.vault.core.entry(stem).or_insert(rf);
        }
        for (stem, rf) in crate::default_lexicon() {
            self.vault.lexicon.entry(stem).or_insert(rf);
        }
        Ok(())
    }

    fn log(&self, at: Timestamp, action: &str, subject: &str, reason: &str) {
        self.log_fact(at, action, subject, reason, None);
    }

    fn log_fact(&self, at: Timestamp, action: &str, subject: &str, reason: &str, fact: Option<&str>) {
        let e = JournalEntry { at, action: action.into(), subject: subject.into(), reason: reason.into(), fact: fact.map(String::from) };
        self.journal.borrow_mut().push(e.clone());
        if let Some(d) = self.deferred.borrow_mut().as_mut() {
            d.push(e);
            return;
        }
        self.write_log(&e);
    }

    fn write_log(&self, e: &JournalEntry) {
        if self.logging {
            // The log is best effort: a failed write must not lose the fact itself.
            let _ = self.vault.log(e.at, &e.action, &e.subject, &e.reason);
        }
    }

    pub fn journal_len(&self) -> usize {
        self.journal.borrow().len()
    }

    /// Changes since a mark from [`KnowledgeBase::journal_len`].
    pub fn journal_since(&self, mark: usize) -> Vec<JournalEntry> {
        self.journal.borrow().get(mark..).map(|s| s.to_vec()).unwrap_or_default()
    }

    /// Run `f` as one all-or-nothing change: if it returns `Err`, the vault is
    /// restored as it was, its journal entries are dropped and nothing reaches
    /// the log. Nested calls join the outer transaction.
    pub fn transaction<T, E>(&mut self, f: impl FnOnce(&mut Self) -> Result<T, E>) -> Result<T, E> {
        if self.deferred.borrow().is_some() {
            return f(self);
        }
        let snapshot = self.vault.snapshot();
        let mark = self.journal_len();
        *self.deferred.borrow_mut() = Some(Vec::new());
        let out = f(self);
        let pending = self.deferred.borrow_mut().take().unwrap_or_default();
        match out {
            Ok(v) => {
                for e in &pending {
                    self.write_log(e);
                }
                Ok(v)
            }
            Err(e) => {
                self.vault.restore(snapshot);
                self.journal.borrow_mut().truncate(mark);
                Err(e)
            }
        }
    }

    /// Public log entry point for other components (learner, agent).
    pub fn log_change(&self, at: Timestamp, action: &str, subject: &str, reason: &str) {
        self.log(at, action, subject, reason);
    }

    /// `functional(P, K)` declarations from core: key length per predicate.
    pub fn functional(&self) -> HashMap<String, usize> {
        let mut out = HashMap::new();
        for rf in self.vault.core.values() {
            for r in rf.doc.rules() {
                if r.head.pred == "functional" && r.body.is_empty() && r.head.args.len() == 2 {
                    if let (Term::Val(Value::Sym(p)), Term::Val(Value::Int(k))) = (&r.head.args[0], &r.head.args[1]) {
                        out.insert(p.clone(), *k as usize);
                    }
                }
            }
        }
        out
    }

    /// Core predicate names with arity.
    pub fn core_predicates(&self) -> Vec<(String, usize)> {
        let mut out = Vec::new();
        for rf in self.vault.core.values() {
            for r in rf.doc.rules() {
                if r.head.pred == "core_predicate" && r.body.is_empty() && r.head.args.len() >= 2 {
                    if let (Term::Val(Value::Sym(p)), Term::Val(Value::Int(k))) = (&r.head.args[0], &r.head.args[1]) {
                        out.push((p.clone(), *k as usize));
                    }
                }
            }
        }
        out
    }

    pub fn all_facts(&self) -> Vec<Fact> {
        self.vault.all_facts().into_iter().map(|(_, f)| f).collect()
    }

    pub fn facts_at(&self, t: Timestamp) -> Vec<Fact> {
        self.all_facts().into_iter().filter(|f| f.valid_at(t)).collect()
    }

    /// Every stored fact matching `pattern`, current or closed, oldest first.
    pub fn history(&self, pattern: &Atom) -> Vec<Fact> {
        let mut v: Vec<Fact> = self.all_facts().into_iter().filter(|f| matches(pattern, &f.atom)).collect();
        v.sort_by_key(|f| f.from.unwrap_or(Timestamp(i64::MIN)));
        v
    }

    /// Facts open (no end time) whose atom satisfies `pred`.
    fn open_matching(&self, pred: impl Fn(&GroundAtom) -> bool) -> Vec<Fact> {
        self.all_facts().into_iter().filter(|f| f.is_open() && pred(&f.atom)).collect()
    }

    /// Store a fact at time `at`. Rules:
    /// - the same open fact already exists: raise its confidence;
    /// - an open fact states the opposite (`p` vs `-p`): close it at `at`;
    /// - a functional predicate gets a new value for the same key: close the old value.
    pub fn assert_fact(&mut self, mut fact: Fact, at: Timestamp, reason: &str) -> AssertOutcome {
        if fact.from.is_none() {
            fact.from = Some(at);
        }
        let atom = fact.atom.clone();

        // Already known and current?
        if let Some(existing) = self.open_matching(|g| *g == atom).into_iter().next() {
            let new_conf = confirm_conf(existing.conf, fact.conf);
            self.vault.update_facts(|f| {
                if f.atom == atom && f.is_open() && f.conf != new_conf {
                    f.conf = new_conf;
                    true
                } else {
                    false
                }
            });
            self.log(at, "confirm", &atom.to_string(), reason);
            return AssertOutcome::Confirmed { conf: new_conf };
        }

        // What does this fact end?
        let complement = atom.complement();
        let functional = self.functional();
        // A thing has one colour or size at a time, but many qualities (soft and sweet).
        let many_valued = atom.base_pred() == "property" && atom.args.get(1).and_then(|v| v.as_sym()) == Some("quality");
        let key_len = if many_valued { None } else { functional.get(atom.base_pred()).copied() };
        let negated = atom.is_strong_negation();
        let conflicts = |g: &GroundAtom| -> bool {
            if *g == complement {
                return true;
            }
            if let Some(k) = key_len {
                // Only positive values compete for a functional key.
                !negated
                    && g.pred == atom.pred
                    && g.args.len() == atom.args.len()
                    && k < atom.args.len()
                    && g.args[..k] == atom.args[..k]
                    && g.args[k..] != atom.args[k..]
            } else {
                false
            }
        };
        let to_close = self.open_matching(&conflicts);
        let from = fact.from.unwrap_or(at);
        if !to_close.is_empty() {
            self.vault.update_facts(|f| {
                if f.is_open() && conflicts(&f.atom) {
                    f.until = Some(from);
                    true
                } else {
                    false
                }
            });
            for c in &to_close {
                self.log(at, "close", &c.atom.to_string(), &format!("contradicted by {atom}"));
            }
        }
        if fact.id.is_none() {
            fact.id = Some(self.new_fact_id());
        }
        self.log_fact(at, "add", &atom.to_string(), reason, fact.id.as_deref());
        self.vault.add_fact(fact);
        let closed = to_close
            .into_iter()
            .map(|mut f| {
                f.until = Some(from);
                f
            })
            .collect();
        AssertOutcome::Added { closed }
    }

    /// "It used to be true": true before `at`, not now. A current copy of the
    /// fact is closed at `at` (it is no longer true); if there is none, a
    /// closed fact is stored. Returns the facts that were closed.
    pub fn assert_past(&mut self, mut fact: Fact, at: Timestamp, reason: &str) -> Vec<Fact> {
        let atom = fact.atom.clone();
        let closed = self.close_matching(&atom.to_atom(), at, reason);
        if closed.is_empty() {
            let already = self.all_facts().iter().any(|f| f.atom == atom && f.until.map_or(false, |u| u <= at));
            if !already {
                fact.from = None;
                fact.until = Some(at);
                if fact.id.is_none() {
                    fact.id = Some(self.new_fact_id());
                }
                self.log_fact(at, "add_past", &atom.to_string(), reason, fact.id.as_deref());
                self.vault.add_fact(fact);
            }
        }
        closed
    }

    /// End every open fact matching `pattern` at `at` (without asserting the opposite).
    pub fn close_matching(&mut self, pattern: &Atom, at: Timestamp, reason: &str) -> Vec<Fact> {
        let closed = self.open_matching(|g| matches(pattern, g));
        if !closed.is_empty() {
            self.vault.update_facts(|f| {
                if f.is_open() && matches(pattern, &f.atom) {
                    f.until = Some(at);
                    true
                } else {
                    false
                }
            });
            for c in &closed {
                self.log(at, "close", &c.atom.to_string(), reason);
            }
        }
        closed
    }

    /// Change the confidence of current facts equal to `atom`.
    pub fn set_conf(&mut self, atom: &GroundAtom, conf: f64, at: Timestamp, reason: &str) -> usize {
        let n = self.vault.update_facts(|f| {
            if f.atom == *atom && f.is_open() && f.conf != conf {
                f.conf = conf;
                true
            } else {
                false
            }
        });
        if n > 0 {
            self.log(at, "set_conf", &format!("{atom} -> {}", crate::term::fmt_conf(conf)), reason);
        }
        n
    }

    /// Count a use of each of these atoms (given facts behind an answer).
    /// Counts go to `stats/`, not to the fact files: answering never rewrites knowledge.
    pub fn record_use(&mut self, atoms: &[GroundAtom], at: Timestamp) {
        for a in atoms {
            self.vault.record_use(&a.to_string(), at);
        }
    }

    /// Uses of an atom: counted in `stats/` plus any old `@uses` on its fact lines.
    pub fn uses(&self, atom: &GroundAtom) -> crate::vault::UseStat {
        let mut u = self.vault.stats.get(&atom.to_string()).copied().unwrap_or_default();
        for f in self.all_facts().iter().filter(|f| f.atom == *atom) {
            u.uses += f.uses;
            if u.last.is_none() {
                u.last = f.last_used;
            }
        }
        u
    }

    /// All rules for the reasoner: core rules (conf 1) and learned rules
    /// that are not rejected. Hypotheses are included only if asked.
    pub fn rules(&self, include_hypotheses: bool) -> Vec<LoadedRule> {
        let mut out = Vec::new();
        for (stem, rf) in &self.vault.core {
            for (i, r) in rf.doc.rules().enumerate() {
                if r.head.pred == "core_predicate" || r.head.pred == "functional" {
                    continue;
                }
                out.push(LoadedRule {
                    id: format!("core/{stem}#{i}"),
                    rule: r.clone(),
                    conf: 1.0,
                    status: RuleStatus::Confirmed,
                    module: "core".into(),
                });
            }
        }
        for (stem, rf) in &self.vault.rules {
            let m = rf.meta();
            let ok = match m.status {
                RuleStatus::Confirmed => true,
                RuleStatus::Hypothesis => include_hypotheses,
                RuleStatus::Rejected => false,
            };
            if !ok {
                continue;
            }
            for (i, r) in rf.doc.rules().enumerate() {
                out.push(LoadedRule {
                    id: format!("{stem}#{i}"),
                    rule: r.clone(),
                    conf: m.conf,
                    status: m.status,
                    module: m.module.clone(),
                });
            }
        }
        out
    }

    /// Find the rule file holding an exactly equal rule.
    pub fn find_rule(&self, rule: &Rule) -> Option<String> {
        self.vault
            .rules
            .iter()
            .find(|(_, rf)| rf.doc.items().any(|i| matches!(i, Item::Rule(r) if r == rule)))
            .map(|(s, _)| s.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{parse_atom, parse_ground};

    fn kb() -> KnowledgeBase {
        let mut v = Vault::empty("unused");
        for (s, rf) in crate::default_core() {
            v.core.insert(s, rf);
        }
        let mut kb = KnowledgeBase::new(v);
        kb.logging = false;
        kb
    }

    fn fact(s: &str) -> Fact {
        Fact::new(parse_ground(s).unwrap(), "test")
    }

    #[test]
    fn negation_closes_old_fact() {
        let mut kb = kb();
        let t1 = Timestamp::from_ymd(2026, 1, 1);
        let t2 = Timestamp::from_ymd(2026, 6, 1);
        kb.assert_fact(fact("likes(maksim, coffee)"), t1, "told");
        let out = kb.assert_fact(fact("-likes(maksim, coffee)"), t2, "told");
        match out {
            AssertOutcome::Added { closed } => assert_eq!(closed.len(), 1),
            o => panic!("{o:?}"),
        }
        let now = t2.plus_days(1);
        assert!(kb.facts_at(now).iter().all(|f| f.atom.to_string() != "likes(maksim, coffee)"));
        assert!(kb.facts_at(now).iter().any(|f| f.atom.to_string() == "-likes(maksim, coffee)"));
        assert!(kb.facts_at(t1.plus_days(1)).iter().any(|f| f.atom.to_string() == "likes(maksim, coffee)"));
        let h = kb.history(&parse_atom("likes(maksim, X)").unwrap());
        assert_eq!(h.len(), 1);
        assert_eq!(h[0].until, Some(t2));
    }

    #[test]
    fn functional_value_replaced() {
        let mut kb = kb();
        let t1 = Timestamp::from_ymd(2026, 1, 1);
        let t2 = Timestamp::from_ymd(2026, 2, 1);
        kb.assert_fact(fact("property(car, colour, red)"), t1, "x");
        kb.assert_fact(fact("property(car, size, big)"), t1, "x");
        kb.assert_fact(fact("property(car, colour, blue)"), t2, "x");
        let now: Vec<String> = kb.facts_at(t2).iter().map(|f| f.atom.to_string()).collect();
        assert!(now.contains(&"property(car, colour, blue)".to_string()));
        assert!(now.contains(&"property(car, size, big)".to_string()));
        assert!(!now.contains(&"property(car, colour, red)".to_string()));
    }

    #[test]
    fn qualities_are_not_functional() {
        let mut kb = kb();
        let t = Timestamp::from_ymd(2026, 1, 1);
        kb.assert_fact(fact("property(pear, quality, soft)"), t, "x");
        kb.assert_fact(fact("property(pear, quality, sweet)"), t.plus_days(1), "x");
        assert_eq!(kb.facts_at(t.plus_days(2)).len(), 2);
    }

    #[test]
    fn repeat_confirms() {
        let mut kb = kb();
        let t = Timestamp::from_ymd(2026, 1, 1);
        kb.assert_fact(fact("likes(ann, tea)").with_conf(0.6), t, "x");
        let o = kb.assert_fact(fact("likes(ann, tea)").with_conf(0.6), t, "x");
        assert_eq!(o, AssertOutcome::Confirmed { conf: 0.8 });
        assert_eq!(kb.all_facts().len(), 1);
    }

    #[test]
    fn usage_tracking() {
        let mut kb = kb();
        let t = Timestamp::from_ymd(2026, 1, 1);
        kb.assert_fact(fact("likes(ann, tea)"), t, "x");
        kb.record_use(&[parse_ground("likes(ann, tea)").unwrap()], t);
        kb.record_use(&[parse_ground("likes(ann, tea)").unwrap()], t);
        let gen = kb.vault.generation;
        kb.record_use(&[parse_ground("likes(ann, tea)").unwrap()], t);
        assert_eq!(kb.vault.generation, gen, "a use is not a change of knowledge");
        let u = kb.uses(&parse_ground("likes(ann, tea)").unwrap());
        assert_eq!(u.uses, 3);
        assert_eq!(u.last, Some(t));
        assert_eq!(kb.all_facts()[0].uses, 0, "the fact line is untouched");
    }

    #[test]
    fn transaction_is_all_or_nothing() {
        let mut kb = kb();
        let t = Timestamp::from_ymd(2026, 1, 1);
        kb.assert_fact(fact("likes(ann, tea)"), t, "x");
        let gen = kb.vault.generation;
        let mark = kb.journal_len();
        let r: Result<(), &str> = kb.transaction(|kb| {
            kb.assert_fact(fact("-likes(ann, tea)"), t.plus_days(1), "x");
            kb.assert_fact(fact("likes(bob, tea)"), t.plus_days(1), "x");
            Err("refused")
        });
        assert!(r.is_err());
        let now: Vec<String> = kb.facts_at(t.plus_days(2)).iter().map(|f| f.atom.to_string()).collect();
        assert_eq!(now, vec!["likes(ann, tea)"], "nothing of the failed change is left");
        assert!(kb.journal_since(mark).is_empty());
        assert!(kb.vault.generation > gen, "cached models must notice the restore");
        let ok: Result<(), ()> = kb.transaction(|kb| {
            kb.assert_fact(fact("likes(bob, tea)"), t.plus_days(1), "x");
            Ok(())
        });
        assert!(ok.is_ok());
        assert_eq!(kb.facts_at(t.plus_days(2)).len(), 2);
        assert_eq!(kb.journal_since(mark).len(), 1);
    }

    #[test]
    fn used_to_closes_the_current_fact() {
        let mut kb = kb();
        let t1 = Timestamp::from_ymd(2026, 1, 1);
        let t2 = Timestamp::from_ymd(2026, 3, 1);
        kb.assert_fact(fact("likes(ann, tea)"), t1, "x");
        let closed = kb.assert_past(fact("likes(ann, tea)"), t2, "she used to");
        assert_eq!(closed.len(), 1);
        assert!(kb.facts_at(t2.plus_days(1)).is_empty(), "no longer true now");
        assert_eq!(kb.facts_at(t1.plus_days(1)).len(), 1, "still true back then");
        // A past fact with no current copy is stored closed, once.
        kb.assert_past(fact("likes(bob, tea)"), t2, "x");
        kb.assert_past(fact("likes(bob, tea)"), t2.plus_days(1), "x");
        let h = kb.history(&parse_atom("likes(bob, X)").unwrap());
        assert_eq!(h.len(), 1);
        assert_eq!(h[0].until, Some(t2));
    }

    #[test]
    fn fact_ids() {
        let mut kb = kb();
        let t = Timestamp::from_ymd(2026, 1, 1);
        kb.assert_fact(fact("likes(ann, tea)"), t, "x");
        kb.assert_fact(fact("likes(bob, tea)"), t, "x");
        let ids: Vec<String> = kb.all_facts().iter().filter_map(|f| f.id.clone()).collect();
        assert_eq!(ids.len(), 2);
        assert_ne!(ids[0], ids[1]);
        // The same atom stored in another file by another conversation keeps its own id.
        let mut copy = fact("likes(ann, tea)");
        copy.id = Some("f99".into());
        copy.conv = Some("s2".into());
        kb.vault.add_fact(copy.clone());
        let ann = kb.all_facts().into_iter().find(|f| f.id.as_deref() == Some("f99")).unwrap();
        assert_eq!(ann.conv.as_deref(), Some("s2"));
        // A reviewer closes exactly the fact it judged.
        let closed = kb.close_by_id("f99", t.plus_days(1), "reviewer").unwrap();
        assert_eq!(closed.id.as_deref(), Some("f99"));
        let open: Vec<_> = kb.facts_at(t.plus_days(2)).into_iter().filter(|f| f.atom.to_string() == "likes(ann, tea)").collect();
        assert_eq!(open.len(), 1, "the other copy stays");
        // Ids survive a reload and numbering continues after the highest one.
        assert!(fact("likes(x, y)").with_conf(1.0).to_line().find("@id").is_none());
        let line = kb.fact_by_id(&ids[0]).unwrap().to_line();
        let back = crate::syntax::parse_fact_line(&line).unwrap();
        assert_eq!(back.id, Some(ids[0].clone()));
        let mut kb2 = KnowledgeBase::new(std::mem::take(&mut kb.vault));
        assert_eq!(kb2.new_fact_id(), "f100");
        assert_eq!(kb2.new_event_id(), "e1");
    }

    #[test]
    fn core_loaded() {
        let kb = kb();
        assert!(kb.core_predicates().len() >= 20);
        assert_eq!(kb.functional().get("property"), Some(&2));
        assert!(kb.rules(false).iter().any(|r| r.rule.head.pred == "is_a"));
        assert!(kb.vault.errors().is_empty(), "{:?}", kb.vault.errors());
    }
}
