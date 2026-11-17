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
        KnowledgeBase { vault, logging: true, journal: RefCell::new(Vec::new()), deferred: RefCell::new(None) }
    }

    /// Load a vault from disk, seeding the core ontology if it is missing.
    pub fn open(root: impl Into<PathBuf>) -> io::Result<Self> {
        let mut vault = Vault::load(root)?;
        if vault.core.is_empty() {
            for (stem, rf) in crate::default_core() {
                vault.core.insert(stem, rf);
            }
            vault.save_all()?;
        }
        Ok(KnowledgeBase::new(vault))
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.vault.save()
    }

    pub fn reload(&mut self) -> io::Result<()> {
        self.vault.reload()?;
        if self.vault.core.is_empty() {
            for (stem, rf) in crate::default_core() {
                self.vault.core.insert(stem, rf);
            }
        }
        Ok(())
    }

    fn log(&self, at: Timestamp, action: &str, subject: &str, reason: &str) {
        let e = JournalEntry { at, action: action.into(), subject: subject.into(), reason: reason.into() };
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
        self.log(at, "add", &atom.to_string(), reason);
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
                self.log(at, "add_past", &atom.to_string(), reason);
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

    /// Count a use of each fact valid at `at` with one of these atoms.
    pub fn record_use(&mut self, atoms: &[GroundAtom], at: Timestamp) {
        if atoms.is_empty() {
            return;
        }
        self.vault.update_facts(|f| {
            if f.valid_at(at) && atoms.contains(&f.atom) {
                f.uses += 1;
                f.last_used = Some(at);
                true
            } else {
                false
            }
        });
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
        let f = &kb.all_facts()[0];
        assert_eq!(f.uses, 2);
        assert_eq!(f.last_used, Some(t));
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
    fn core_loaded() {
        let kb = kb();
        assert!(kb.core_predicates().len() >= 20);
        assert_eq!(kb.functional().get("property"), Some(&2));
        assert!(kb.rules(false).iter().any(|r| r.rule.head.pred == "is_a"));
        assert!(kb.vault.errors().is_empty(), "{:?}", kb.vault.errors());
    }
}
