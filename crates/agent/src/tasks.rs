//! Help with tasks (Design 2.0): the acts `request`, `ask_how`, `ask_need`,
//! `ask_duration`, `ask_next`, `done`, `lack`, `inform`, `accept`, `reject`,
//! `feedback`, `teach` and `new_pred`.
//!
//! Milestone 14 gives each act an honest reply from the procedure layer: the
//! steps of a known task, what it needs, how long it takes, the next step,
//! or "teach me" for an unknown task. Milestone 15 replaces this with the
//! dialogue manager (task frames, slots, move rules).

use crate::{Agent, Brain};
use sla_core::{Atom, GroundAtom, Value};
use sla_nlg::Reply;
use sla_reasoner::Model;

/// Words of a symbol (`cherry_tomato` -> "cherry tomato").
pub fn words(s: &str) -> String {
    s.replace('_', " ")
}

fn sym(s: &str) -> Value {
    Value::sym(s)
}

fn g(pred: &str, args: &[&str]) -> GroundAtom {
    GroundAtom::new(pred, args.iter().map(|a| sym(a)).collect())
}

fn values(m: &Model, pred: &str, first: &str, pos: usize) -> Vec<String> {
    let mut out: Vec<String> = m
        .facts_of(pred)
        .into_iter()
        .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(first))
        .filter_map(|a| a.args.get(pos).map(|v| v.as_sym().map(String::from).unwrap_or_else(|| v.to_string())))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn list(items: &[String]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        n => format!("{} and {}", items[..n - 1].join(", "), items[n - 1]),
    }
}

fn number(v: &Value) -> String {
    v.to_string()
}

/// "10 to 12 minutes", "1 minute".
fn range(lo: &Value, hi: &Value, unit: &str) -> String {
    let plural = |v: &Value| if v.as_f64() == Some(1.0) { unit.to_string() } else { format!("{unit}s") };
    if lo == hi {
        format!("{} {}", number(lo), plural(lo))
    } else {
        format!("{} to {} {}", number(lo), number(hi), plural(hi))
    }
}

/// A known task that an unknown one is about, judged by what each involves:
/// the request's object among a task's name, needs and step objects, and its
/// verb among the task's name and step actions (`patch_tube` -> `fix_tire`,
/// which needs a tube and has a step "patch"). Only a clear best counts.
pub(crate) fn involved_task(m: &Model, t: &str, known: &[String]) -> Option<String> {
    let parts: Vec<String> = t.split('_').map(singular).collect();
    let (verb, objs) = match parts.split_first() {
        Some((v, rest)) if !rest.is_empty() => (v.clone(), rest.to_vec()),
        _ => return None,
    };
    let of = |pred: &str, first: &str| -> Vec<String> {
        m.facts_of(pred)
            .into_iter()
            .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(first))
            .filter_map(|a| a.args.get(1)?.as_sym().map(|s| s.split('_').map(singular).collect::<Vec<_>>().join("_")))
            .collect()
    };
    let mut scored: Vec<(usize, &String)> = Vec::new();
    for k in known {
        let name: Vec<String> = k.split('_').map(singular).collect();
        let steps = of("step", k);
        let things: Vec<String> = of("needs", k).into_iter().chain(steps.iter().flat_map(|s| of("object_of", s))).collect();
        let actions: Vec<String> = steps.iter().flat_map(|s| of("action", s)).collect();
        let thing_words: Vec<&str> = things.iter().flat_map(|x| x.split('_')).collect();
        let mut score = 0;
        for o in &objs {
            if name.contains(o) {
                score += 3;
            } else if thing_words.contains(&o.as_str()) {
                score += 2;
            }
        }
        if name.contains(&verb) || actions.contains(&verb) {
            score += 1;
        }
        if score >= 3 {
            scored.push((score, k));
        }
    }
    scored.sort_by_key(|(s, _)| std::cmp::Reverse(*s));
    match scored.as_slice() {
        [(a, k), (b, _), ..] if a > b => Some((*k).clone()),
        [(_, k)] => Some((*k).clone()),
        _ => None,
    }
}

/// "a kettle", "an egg", "water" (mass nouns and plurals take no article).
pub(crate) fn with_article(x: &str) -> String {
    let w = words(x);
    let last = x.rsplit('_').next().unwrap_or(x);
    if sla_nlg::MASS.contains(&x) || sla_nlg::MASS.contains(&last) || (last.ends_with('s') && !last.ends_with("ss")) {
        return w;
    }
    let an = w.starts_with(|c: char| "aeiou".contains(c));
    format!("{} {w}", if an { "an" } else { "a" })
}

/// Things a step works in or into, rather than with.
const CONTAINERS: &[&str] = &[
    "pot", "pan", "frying_pan", "saucepan", "kettle", "mug", "cup", "glass", "bowl", "sink", "bucket", "oven", "baking_tray", "tray", "jar", "blender",
    "washing_machine", "basket", "envelope", "box", "bag", "suitcase", "watering_can", "vase", "dish", "plate", "toaster", "microwave", "fridge", "sieve",
];

/// " in the pot", " into the mug", " with the fork and salt": the items a step
/// uses, each with its own preposition.
pub(crate) fn uses_phrase(action: &str, uses: &[String]) -> String {
    let into = matches!(action, "pour" | "add" | "put" | "place" | "load" | "transfer" | "drop" | "lower" | "tip");
    // You pour from a jug, a watering can or a kettle, not into it.
    const POUR_FROM: &[&str] = &["watering_can", "jug", "kettle", "saucepan", "bottle", "pitcher", "can"];
    let from: Vec<&String> = if action == "pour" { uses.iter().filter(|u| POUR_FROM.contains(&u.as_str())).collect() } else { Vec::new() };
    let uses: Vec<String> = uses.iter().filter(|u| !from.contains(u)).cloned().collect();
    let uses = &uses;
    let (cont, tools): (Vec<&String>, Vec<&String>) = uses.iter().partition(|u| CONTAINERS.contains(&u.as_str()));
    let the = |u: &String| if sla_nlg::MASS.contains(&u.as_str()) { words(u) } else { format!("the {}", words(u)) };
    let mut out = String::new();
    if !cont.is_empty() {
        out.push_str(&format!(" {} {}", if into { "into" } else { "in" }, list(&cont.iter().map(|u| the(u)).collect::<Vec<_>>())));
    }
    // The preposition the tool takes with this action: you remove a wheel from
    // the bike, put a dish on the rack, add detergent to the drawer.
    const SURFACES: &[&str] = &["drying_rack", "rack", "ironing_board", "board", "tray", "baking_tray", "shelf", "table", "plate", "hanger", "line", "washing_line", "wall", "counter", "blanket"];
    const TOOLS: &[&str] = &["lever", "wrench", "driver", "screwdriver", "knife", "fork", "spoon", "scissors", "pliers", "opener", "tongs", "spatula", "hammer", "brush", "cloth", "sponge", "squeegee", "pump", "whisk", "peeler", "hand", "finger"];
    let prep = |u: &String| -> &'static str {
        let surface = SURFACES.contains(&u.as_str()) || u.ends_with("_rack") || u.ends_with("_board");
        let tool = u.split('_').any(|w| TOOLS.contains(&w.trim_end_matches('s')) || TOOLS.contains(&w));
        match action {
            "remove" | "take" | "lift" | "unscrew" | "detach" if !tool => "from",
            "put" | "place" | "hang" | "lay" | "set" | "leave" | "stand" | "spread" | "dry" if surface => "on",
            "add" | "attach" | "stick" | "fix" | "pin" | "tape" | "send" | "give" | "deliver" | "post" => "to",
            _ => "with",
        }
    };
    let mut groups: Vec<(&str, Vec<String>)> = Vec::new();
    for u in &tools {
        let p = prep(u);
        match groups.iter_mut().find(|(q, _)| *q == p) {
            Some((_, v)) => v.push(the(u)),
            None => groups.push((p, vec![the(u)])),
        }
    }
    for (p, v) in groups {
        out.push_str(&format!(" {p} {}", list(&v)));
    }
    if !from.is_empty() {
        out.push_str(&format!(" from {}", list(&from.iter().map(|u| the(u)).collect::<Vec<_>>())));
    }
    out
}

/// Verbs that say only "do the task": `cook_egg` may be `boil_egg`.
const GENERIC: &[&str] = &["make", "cook", "prepare", "do", "get", "have", "fix", "help"];

fn singular(w: &str) -> String {
    if let Some(r) = w.strip_suffix("ies") {
        return format!("{r}y");
    }
    if w.ends_with("oes") || w.ends_with("ches") || w.ends_with("shes") {
        return w[..w.len() - 2].to_string();
    }
    match w.strip_suffix('s') {
        Some(r) if !w.ends_with("ss") && r.len() > 2 => r.to_string(),
        _ => w.to_string(),
    }
}

/// A known task that a differently worded task name means: the same thing
/// done, and the same verb or a generic one (`make_potato` -> `bake_potato`,
/// `make_garlic_tomato_pasta` -> `garlic_tomato_pasta`). Only an unambiguous
/// match counts.
pub(crate) fn similar_task<'a>(t: &str, known: impl Iterator<Item = &'a str>) -> Option<String> {
    let split = |x: &str| -> (String, String) {
        match x.split_once('_') {
            Some((v, rest)) => (v.to_string(), rest.split('_').map(singular).collect::<Vec<_>>().join("_")),
            None => (String::new(), singular(x)),
        }
    };
    let (tv, to) = split(t);
    let whole = t.split('_').map(singular).collect::<Vec<_>>().join("_");
    let mut hits: Vec<&str> = Vec::new();
    for k in known {
        if k == t {
            return Some(k.to_string());
        }
        let (kv, ko) = split(k);
        let kwhole = k.split('_').map(singular).collect::<Vec<_>>().join("_");
        let same_thing = (!to.is_empty() && to == ko && (tv == kv || GENERIC.contains(&tv.as_str()) || GENERIC.contains(&kv.as_str())))
            || (GENERIC.contains(&tv.as_str()) && to == kwhole)
            || (GENERIC.contains(&kv.as_str()) && ko == whole);
        if same_thing && !hits.contains(&k) {
            hits.push(k);
        }
    }
    (hits.len() == 1).then(|| hits[0].to_string())
}

impl Agent {
    /// The task a request names: itself if known, a known task that solves
    /// it (a feasible one first), or one said to be the same.
    pub(crate) fn resolve_task(&self, m: &Model, t: &str) -> Option<String> {
        if m.contains(&g("task", &[t])) {
            return Some(t.to_string());
        }
        let mut solvers: Vec<GroundAtom> = m.facts_of("solves").into_iter().filter(|a| a.args.get(1).and_then(|v| v.as_sym()) == Some(t)).collect();
        solvers.sort_by_key(|a| !m.contains(&g("feasible", &[a.args[0].as_sym().unwrap_or("")])));
        if let Some(a) = solvers.first() {
            return a.args[0].as_sym().map(String::from);
        }
        for a in m.facts_of("same") {
            if a.args.first().and_then(|v| v.as_sym()) == Some(t) {
                if let Some(o) = a.args.get(1).and_then(|v| v.as_sym()) {
                    if m.contains(&g("task", &[o])) {
                        return Some(o.to_string());
                    }
                }
            }
        }
        let known: Vec<String> = m.facts_of("task").iter().filter_map(|a| a.args.first().and_then(|v| v.as_sym()).map(String::from)).collect();
        similar_task(t, known.iter().map(String::as_str)).or_else(|| involved_task(m, t, &known))
    }

    /// The model with this conversation's situation facts on top of the vault.
    pub(crate) fn session_model(&self, b: &mut Brain) -> std::sync::Arc<Model> {
        let now = self.now();
        if self.session_facts.is_empty() {
            return b.model_at(now);
        }
        b.model_with(now, &self.session_facts)
    }

    fn teach_me(&self, t: &str) -> Reply {
        Reply { text: format!("I don't know how to {} yet. What would a good answer include?", words(t)), intent: "teach_me", claims: Vec::new() }
    }

    /// One step as an instruction: "Boil the pasta (10 to 12 minutes)."
    pub(crate) fn step_text(&self, m: &Model, s: &str) -> (String, Vec<GroundAtom>) {
        let mut claims = Vec::new();
        let action = values(m, "action", s, 1).into_iter().next();
        let object = values(m, "object_of", s, 1).into_iter().next();
        // A count noun is plural when the user spoke of it in the plural
        // ("I have cherry tomatoes" -> "Add the cherry tomatoes"); otherwise
        // singular ("Heat the oven").
        let count = |o: &str| !sla_nlg::MASS.contains(&o) && {
            let pl = sla_nlg::plural(&words(o));
            self.heard_text.split(|c: char| !c.is_alphanumeric() && c != ' ').any(|part| format!(" {part} ").contains(&format!(" {pl} ")))
        };
        let mut text = match (&action, &object) {
            (Some(a), Some(o)) => {
                claims.push(g("action", &[s, a]));
                claims.push(g("object_of", &[s, o]));
                let a = words(a);
                let mut c = a.chars();
                let a = c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default();
                let thing = if count(o) { sla_nlg::plural(&words(o)) } else { words(o) };
                format!("{a} the {thing}")
            }
            (Some(a), None) => {
                claims.push(g("action", &[s, a]));
                let a = words(a);
                let mut c = a.chars();
                c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
            }
            _ => words(s),
        };
        let uses = values(m, "uses", s, 1);
        if !uses.is_empty() {
            for u in &uses {
                claims.push(g("uses", &[s, u]));
            }
            // Not the step's own object again ("heat the oven in the oven").
            let shown: Vec<String> = uses.iter().filter(|u| Some(*u) != object.as_ref()).cloned().collect();
            text.push_str(&uses_phrase(action.as_deref().unwrap_or(""), &shown));
        }
        if let Some(u) = values(m, "until", s, 1).into_iter().next().filter(|u| Some(u) != action.as_ref()) {
            claims.push(g("until", &[s, &u]));
            let they = object.as_deref().map_or(false, count);
            // An end state that is a verb reads as one ("until it drains"), others as a state.
            const STATE_VERBS: &[&str] = &["drain", "boil", "melt", "simmer", "dry", "bubble", "foam", "thicken", "set", "rise", "stop", "run", "fizz", "drip"];
            if STATE_VERBS.contains(&u.as_str()) && u != "dry" && u != "set" {
                let v = if they { u.clone() } else if u.ends_with('h') || u.ends_with('s') || u.ends_with('z') { format!("{u}es") } else { format!("{u}s") };
                text.push_str(&format!(" until {} {}", if they { "they" } else { "it" }, v));
            } else {
                text.push_str(&format!(" until {} {}", if they { "they are" } else { "it is" }, words(&u)));
            }
        }
        // A step that runs during another: "While you boil the pasta, fry the garlic."
        if let Some(r) = values(m, "overlaps", s, 1).into_iter().next() {
            claims.push(g("overlaps", &[s, &r]));
            let ra = values(m, "action", &r, 1).into_iter().next();
            let ro = values(m, "object_of", &r, 1).into_iter().next();
            if let Some(ra) = ra {
                let obj = ro.map(|o| format!(" the {}", if count(&o) { sla_nlg::plural(&words(&o)) } else { words(&o) })).unwrap_or_default();
                let mut c = text.chars();
                let lower = c.next().map(|f| f.to_lowercase().collect::<String>() + c.as_str()).unwrap_or_default();
                text = format!("While you {ra}{obj}, {lower}");
            }
        }
        if let Some(a) = m.facts_of("takes_time").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(s)) {
            if let Some(unit) = a.args[3].as_sym() {
                text.push_str(&format!(" ({})", range(&a.args[1], &a.args[2], unit)));
            }
            claims.push(a);
        }
        (format!("{text}."), claims)
    }

    /// Steps of a task in an order that respects `before` (ties by name).
    fn ordered_steps(&self, m: &Model, task: &str) -> Vec<String> {
        let mut steps = values(m, "step", task, 1);
        let before = |a: &str, b: &str| m.contains(&g("before", &[a, b]));
        // Insertion sort by the partial order.
        let mut out: Vec<String> = Vec::new();
        steps.sort();
        while !steps.is_empty() {
            let i = steps.iter().position(|s| !steps.iter().any(|o| o != s && before(o, s))).unwrap_or(0);
            out.push(steps.remove(i));
        }
        out
    }

    pub(crate) fn task_help(&mut self, b: &mut Brain, t: &str) -> Reply {
        let m = self.session_model(b);
        let Some(task) = self.resolve_task(&m, t) else { return self.teach_me(t) };
        self.active_task = Some(task.clone());
        let steps = self.ordered_steps(&m, &task);
        if steps.is_empty() {
            return self.teach_me(t);
        }
        let mut claims = vec![g("task", &[&task])];
        let mut lines = Vec::new();
        let missing = values(&m, "missing", &task, 1);
        let lead = if task == t || self.is_verb_phrase(b, &task) { format!("Here is how to {}:", self.task_phrase(b, &task)) } else { format!("You could make {}:", words(&task)) };
        // Transfer: an item the user lacks, with a known substitute, is replaced
        // in the steps and the needs ("Boil the water in the saucepan").
        let subs = self.substitutions(&m, &task);
        let swap = |t: &str| subs.iter().fold(t.to_string(), |acc, (x, sub)| swap_word(&acc, &words(x), &words(sub)));
        // A step that acts on the replaced item with a tool nothing else needs
        // ("hammer the nail" with the hammer) no longer applies; nor does its tool.
        let tools_of = |st: &String| values(&m, "uses", st, 1);
        let mut dropped_tools: Vec<String> = Vec::new();
        let kept: Vec<&String> = steps
            .iter()
            .filter(|st| {
                let on_replaced = values(&m, "object_of", st, 1).iter().any(|o| subs.iter().any(|(x, _)| x == o));
                // The action is the tool itself ("hammer" with the hammer), and no other step needs it.
                let action = values(&m, "action", st, 1).into_iter().next().unwrap_or_default();
                let only_here: Vec<String> = tools_of(st)
                    .into_iter()
                    .chain(values(&m, "needs", &task, 1).into_iter().filter(|n| n == &action))
                    .filter(|t| t.split('_').next() == Some(action.as_str()))
                    .filter(|t| !subs.iter().any(|(x, _)| x == t) && steps.iter().filter(|o| o != st).all(|o| !tools_of(o).contains(t)))
                    .collect();
                if on_replaced && !only_here.is_empty() {
                    dropped_tools.extend(only_here);
                    return false;
                }
                true
            })
            .collect();
        let extra = self.extra_ingredient(&m, &task, &kept);
        let numbered = kept.len() + extra.is_some() as usize > 3;
        let mut texts: Vec<String> = Vec::new();
        for s in kept.iter() {
            let (text, c) = self.step_text(&m, s);
            claims.extend(c);
            texts.push(swap(&text));
        }
        if let Some(line) = &extra {
            let at = texts.len().saturating_sub(1);
            texts.insert(at, line.clone());
        }
        for (i, text) in texts.into_iter().enumerate() {
            lines.push(if numbered { format!("{}. {text}", i + 1) } else { text });
        }
        // What the task needs comes first.
        let needs = values(&m, "needs", &task, 1);
        let lead = if needs.is_empty() {
            lead
        } else {
            for x in &needs {
                claims.push(g("needs", &[&task, x]));
            }
            let named: Vec<String> = needs
                .iter()
                .filter(|x| !dropped_tools.contains(x))
                .map(|x| match subs.iter().find(|(y, _)| y == x) {
                    Some((_, sub)) => format!("{} (instead of {})", with_article(sub), with_article(x)),
                    None => with_article(x),
                })
                .collect();
            format!("{lead} You need {}.", list(&named))
        };
        let mut text = format!("{lead} {}", lines.join(if numbered { "\n" } else { " " }));
        if numbered {
            text = format!("{lead}\n{}", lines.join("\n"));
        }
        // A stated duration wins over the sum of the steps.
        let stated = m.facts_of("takes_time").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task.as_str()));
        if let Some(a) = stated {
            let unit = a.args[3].as_sym().unwrap_or("minute").to_string();
            text.push_str(&format!(" It takes {} in all.", range(&a.args[1], &a.args[2], &unit)));
            claims.push(a);
        } else if let Some(a) = m.facts_of("total_minutes").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task.as_str())) {
            text.push_str(&format!(" It takes about {} in all.", range(&a.args[1], &a.args[2], "minute")));
            claims.push(a);
        }
        // Not what is replaced or known to be lacking: the notes below say those.
        let lacking_now: Vec<String> = self.session_facts.iter().filter(|f| f.pred == "-available").filter_map(|f| f.args.first()?.as_sym().map(String::from)).collect();
        // Only the task's own needs, and not what the user has in some form ("tap water").
        let have_now: Vec<String> = self.session_facts.iter().filter(|f| f.pred == "available").filter_map(|f| f.args.first()?.as_sym().map(String::from)).collect();
        let needs_now = values(&m, "needs", &task, 1);
        let missing: Vec<String> = missing
            .into_iter()
            .filter(|x| needs_now.contains(x))
            .filter(|x| !have_now.iter().any(|h| h.split('_').any(|w| w == x.as_str()) || x.split('_').any(|w| w == h.as_str())))
            .filter(|x| !subs.iter().any(|(y, _)| y == x) && !lacking_now.contains(x) && !dropped_tools.contains(x))
            .collect();
        if !missing.is_empty() && self.session_facts.iter().any(|f| f.pred == "available") {
            text.push_str(&format!(" You still need {}.", list(&missing.iter().map(|x| words(x)).collect::<Vec<_>>())));
            claims.extend(missing.iter().map(|x| g("missing", &[&task, x])));
        }
        text.push_str(&self.constraint_notes(&m, &task));
        // A friend's close, unless the reply ends by asking something.
        if !text.trim_end().ends_with('?') {
            text.push_str(" Let me know if any step is unclear.");
        }
        self.last_step = steps.first().cloned();
        Reply { text, intent: "answer", claims }
    }

    /// "a cheese omelette": an item the user names with the dish that the task
    /// does not use goes in just before the last step, over what the step
    /// before cooked ("Sprinkle the cheese over the eggs once they are almost
    /// set, before you fold the omelette").
    fn extra_ingredient(&self, m: &Model, task: &str, kept: &[&String]) -> Option<String> {
        let dish = task.rsplit('_').next()?;
        let last = self.heard_text.rsplit('\n').next().unwrap_or("");
        let ws: Vec<&str> = last.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).collect();
        let pos = ws.iter().position(|w| *w == dish || w.trim_end_matches('s') == dish || *w == format!("{dish}es"))?;
        let x = ws.get(pos.checked_sub(1)?)?.to_string();
        // A thing, not a manner ("perfect", "quick", "soft"): an item some procedure
        // uses or a mass noun, and not one this task already uses.
        let known: Vec<String> = ["uses", "needs", "object_of"].iter().flat_map(|p| m.facts_of(p)).filter_map(|a| a.args.get(1).and_then(|v| v.as_sym()).map(String::from)).collect();
        if !(known.contains(&x) || sla_nlg::MASS.contains(&x.as_str())) {
            return None;
        }
        let own: Vec<String> = values(m, "needs", task, 1).into_iter().chain(kept.iter().flat_map(|s| ["uses", "object_of"].iter().flat_map(|p| values(m, p, s, 1)).collect::<Vec<_>>())).collect();
        if own.iter().any(|o| o == &x || o.split('_').any(|w| w == x)) || kept.len() < 2 {
            return None;
        }
        let fin = kept.last()?;
        let prev = kept.get(kept.len() - 2)?;
        let fin_action = values(m, "action", fin, 1).into_iter().next()?;
        let fin_obj = values(m, "object_of", fin, 1).into_iter().next().map(|o| format!(" the {}", words(&o))).unwrap_or_default();
        let prev_obj = values(m, "object_of", prev, 1).into_iter().next()?;
        let they = !sla_nlg::MASS.contains(&prev_obj.as_str());
        let po = if they { sla_nlg::plural(&words(&prev_obj)) } else { words(&prev_obj) };
        let once = values(m, "until", prev, 1).into_iter().next().map(|u| format!(" once {} almost {}", if they { "they are" } else { "it is" }, words(&u))).unwrap_or_default();
        Some(format!("Sprinkle the {} over the {po}{once}, before you {fin_action}{fin_obj}.", words(&x)))
    }

    /// (item the user lacks, its substitute) for the items `task` needs or its
    /// steps use; a substitute the user has is preferred.
    pub(crate) fn substitutions(&self, m: &Model, task: &str) -> Vec<(String, String)> {
        self.substitutions_why(m, task).into_iter().map(|(x, sub, _)| (x, sub)).collect()
    }

    /// Like `substitutions`, with how each was found: `None` for a taught
    /// substitute, `Some(other_task)` for a guess by analogy: another known
    /// procedure does the same action (on the same object, if it can) with an
    /// item the user has ("boil water in a pot" when boiling an egg, for a kettle).
    pub(crate) fn substitutions_why(&self, m: &Model, task: &str) -> Vec<(String, String, Option<String>)> {
        let sym = |a: &GroundAtom, i: usize| a.args.get(i).and_then(|v| v.as_sym()).map(String::from);
        let steps = self.ordered_steps(m, task);
        let used: Vec<String> = m
            .facts_of("needs")
            .into_iter()
            .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task))
            .filter_map(|a| sym(&a, 1))
            .chain(steps.iter().flat_map(|st| values(m, "uses", st, 1)))
            .collect();
        let have: Vec<String> = self.session_facts.iter().filter(|a| a.pred == "available").filter_map(|a| sym(a, 0)).collect();
        let lack: Vec<String> = self.session_facts.iter().filter(|a| a.pred == "-available").filter_map(|a| sym(a, 0)).collect();
        let mut out = Vec::new();
        for x in lack.iter().filter(|x| used.contains(x)) {
            let subs: Vec<String> = m.facts_of("substitute").into_iter().filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(x.as_str())).filter_map(|a| sym(&a, 1)).collect();
            if let Some(sub) = subs.iter().find(|s| have.contains(s)).or(subs.first()) {
                out.push((x.clone(), sub.clone(), None));
                continue;
            }
            // Analogy: the steps of this task that use `x`, and their action and object.
            let mine: Vec<(Option<String>, Option<String>)> = steps
                .iter()
                .filter(|st| values(m, "uses", st, 1).contains(x) || values(m, "object_of", st, 1).contains(x))
                .map(|st| (values(m, "action", st, 1).into_iter().next(), values(m, "object_of", st, 1).into_iter().next().filter(|o| o != x)))
                .collect();
            let mut best: Option<(u8, String, String)> = None;
            for stp in m.facts_of("step") {
                let (Some(other), Some(st2)) = (sym(&stp, 0), sym(&stp, 1)) else { continue };
                if other == task {
                    continue;
                }
                let act2 = values(m, "action", &st2, 1).into_iter().next();
                let obj2 = values(m, "object_of", &st2, 1).into_iter().next();
                for (act, obj) in &mine {
                    if act.is_none() || act != &act2 {
                        continue;
                    }
                    let score = if obj.is_some() && obj == &obj2 { 2 } else { 1 };
                    for y in values(m, "uses", &st2, 1) {
                        if &y != x && have.contains(&y) && !used.contains(&y) && best.as_ref().map_or(true, |b| b.0 < score) {
                            best = Some((score, y.clone(), other.clone()));
                        }
                    }
                }
            }
            if let Some((_, y, other)) = best {
                out.push((x.clone(), y, Some(other)));
                continue;
            }
            // Asked "what do you have that could do the job?", the user named
            // something the task does not otherwise use: their proposal.
            {
                // Not something the task already uses, nor a form of it ("tap water" for water).
                let used_words: Vec<&str> = used.iter().flat_map(|u| u.split('_')).collect();
                let offered: Vec<&String> = have.iter().filter(|y| !used.contains(y) && *y != x && !y.split('_').any(|w| used_words.contains(&w))).collect();
                // Several: the one that shares a stem with the missing item
                // ("tumble dryer" for the drying rack).
                let stem = |w: &str| w.chars().take(3).collect::<String>();
                let akin: Vec<&String> = offered.iter().copied().filter(|y| y.split('_').any(|w| w.len() > 3 && x.split('_').any(|xw| xw.len() > 3 && stem(xw) == stem(w)))).collect();
                match (offered.as_slice(), akin.as_slice()) {
                    ([y], _) | (_, [y]) => out.push((x.clone(), (**y).clone(), Some(String::new()))),
                    _ => {}
                }
            }
        }
        out
    }

    /// Does a task name start with a verb (`boil_egg`), or name a thing (`garlic_tomato_pasta`)?
    pub(crate) fn is_verb_phrase(&self, _b: &Brain, task: &str) -> bool {
        // `boil_egg`, `change_bike_tire`: the first word is a verb. A name that
        // starts with a thing (`garlic_tomato_pasta`) is made.
        let first = task.split('_').next().unwrap_or("");
        self.translator.registry.preds.get(first).map_or(false, |p| p.verb) || matches!(first, "make" | "cook" | "bake" | "boil" | "plan" | "change" | "pack" | "write" | "clean" | "wash" | "fix" | "do")
    }

    /// "boil egg" for a verb phrase, "make garlic tomato pasta" for a thing.
    pub(crate) fn task_phrase(&self, b: &Brain, task: &str) -> String {
        if self.is_verb_phrase(b, task) {
            // "fix a tire", "boil an egg"; mass nouns and plurals as they are.
            match task.split_once('_') {
                Some((verb, rest)) if !rest.contains("_for_") && !rest.contains("_to_") && !matches!(rest, "laundry" | "homework" | "shopping" | "lunch" | "dinner" | "breakfast") => {
                    format!("{} {}", verb, with_article(rest))
                }
                _ => words(task),
            }
        } else {
            format!("make {}", words(task))
        }
    }

    /// Did the user's last message also ask for the steps ("what are the steps", "how do I")?
    pub(crate) fn last_message_asks_steps(&self) -> bool {
        let last = self.heard_text.rsplit('\n').next().unwrap_or("");
        ["step", "how do i", "how do you", "how to", "walk me through", "what do i do", "what do i need to do", "in what order", "where do i start", "how should i", "how can i"].iter().any(|k| last.contains(k))
    }

    /// What the user's situation changes: too little time, or a needed item
    /// they do not have (with no known substitute).
    pub(crate) fn constraint_notes(&self, m: &Model, task: &str) -> String {
        let mut out = String::new();
        let minutes = |n: f64, unit: &str| match unit {
            "hour" => n * 60.0,
            "second" => n / 60.0,
            "day" => n * 1440.0,
            _ => n,
        };
        let have = self.session_facts.iter().find(|a| a.pred == "time_available").and_then(|a| Some((a.args.first()?.as_f64()?, a.args.get(1)?.as_sym()?.to_string())));
        let need = m.facts_of("takes_time").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task)).and_then(|a| Some((a.args.get(1)?.as_f64()?, a.args.get(3)?.as_sym()?.to_string())));
        if let (Some((h, hu)), Some((n, nu))) = (have, need) {
            if minutes(n, &nu) > minutes(h, &hu) {
                let hv = if h.fract() == 0.0 { format!("{}", h as i64) } else { format!("{h}") };
                let unit = if h == 1.0 { hu.clone() } else { format!("{hu}s") };
                out.push_str(&format!(" That is more than the {hv} {unit} you have, and I don't know a quicker way yet."));
            }
        }
        let used: Vec<String> = m
            .facts_of("needs")
            .into_iter()
            .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task))
            .filter_map(|a| a.args.get(1)?.as_sym().map(String::from))
            .chain(self.ordered_steps(m, task).iter().flat_map(|st| values(m, "uses", st, 1)))
            .collect();
        let mut lacking: Vec<String> = self
            .session_facts
            .iter()
            .filter(|a| a.pred == "-available")
            .filter_map(|a| a.args.first()?.as_sym().map(String::from))
            .filter(|x| used.contains(x) && !m.contains(&g("covered", &[x])))
            .collect();
        lacking.dedup();
        // A known substitute: "You can use a pot instead of the kettle."
        let lacking_all: Vec<String> = self
            .session_facts
            .iter()
            .filter(|a| a.pred == "-available")
            .filter_map(|a| a.args.first()?.as_sym().map(String::from))
            .filter(|x| used.contains(x))
            .collect();
        // Also: a needed item the user did not list, while they listed a known
        // substitute for it ("I have lentils" for chili with minced beef).
        let have: Vec<String> = self.session_facts.iter().filter(|a| a.pred == "available").filter_map(|a| a.args.first()?.as_sym().map(String::from)).collect();
        let mut lacking_all = lacking_all;
        if !have.is_empty() {
            for a in m.facts_of("substitute") {
                let (Some(x), Some(sub)) = (a.args.first().and_then(|v| v.as_sym()), a.args.get(1).and_then(|v| v.as_sym())) else { continue };
                if used.iter().any(|u| u == x) && !have.iter().any(|h| h == x) && have.iter().any(|h| h == sub) && !lacking_all.iter().any(|l| l == x) {
                    lacking_all.push(x.to_string());
                }
            }
        }
        // Guesses by analogy (nothing taught): say where the idea comes from.
        let mut guessed: Vec<String> = Vec::new();
        for (x, y, why) in self.substitutions_why(m, task) {
            match why.as_deref() {
                Some("") => {
                    out.push_str(&format!(" Since you have {}, use it instead of the {}.", with_article(&y), words(&x)));
                    guessed.push(x);
                }
                Some(other) => {
                    out.push_str(&format!(" I haven't been taught this, but you could probably use {} instead of the {}: I use {} like that when I {}.", with_article(&y), words(&x), with_article(&y), self.task_phrase_m(other)));
                    guessed.push(x);
                }
                None => {}
            }
        }
        for x in &lacking_all {
            if guessed.contains(x) {
                continue;
            }
            let subs: Vec<String> = m
                .facts_of("substitute")
                .into_iter()
                .filter(|a| a.args.first().and_then(|v| v.as_sym()) == Some(x.as_str()))
                .filter_map(|a| a.args.get(1)?.as_sym().map(String::from))
                .collect();
            // Prefer the substitute the user has.
            let sub = subs.iter().find(|s| have.contains(s)).or(subs.first());
            if let Some(sub) = sub {
                out.push_str(&format!(" You can use {} instead of the {}.", with_article(sub), words(x)));
            }
        }
        lacking.retain(|x| !guessed.contains(x) && !m.facts_of("substitute").iter().any(|a| a.args.first().and_then(|v| v.as_sym()) == Some(x.as_str())));
        if !lacking.is_empty() {
            let l = list(&lacking.iter().map(|x| words(x)).collect::<Vec<_>>());
            let asked = lacking.iter().all(|x| self.asked_lack.borrow().contains(x));
            if asked {
                out.push_str(&format!(" You don't have {l}, and I don't know what can replace it yet."));
            } else {
                // Ask what they have: an answer lets me look for something that does the same job.
                out.push_str(&format!(" You don't have {l}: what do you have that could do the job?"));
                self.asked_lack.borrow_mut().extend(lacking.iter().cloned());
            }
        }
        out
    }

    pub(crate) fn task_needs(&mut self, b: &mut Brain, t: &str) -> Reply {
        let m = self.session_model(b);
        let Some(task) = self.resolve_task(&m, t) else { return self.teach_me(t) };
        // What the user says next ("I have X but no Y") is about this task.
        self.active_task = Some(task.clone());
        let needs = values(&m, "needs_all", &task, 1);
        if needs.is_empty() {
            return self.teach_me(t);
        }
        let claims = needs.iter().map(|x| g("needs_all", &[&task, x])).collect();
        let notes = self.constraint_notes(&m, &task);
        // Not "a frying pan ... and a pan": a bare word covered by a longer item.
        let needs: Vec<String> = needs.iter().filter(|x| !needs.iter().any(|y| y != *x && y.split('_').count() > 1 && y.split('_').any(|w| w == x.as_str()))).cloned().collect();
        let need_list = list(&needs.iter().map(|x| with_article(x)).collect::<Vec<_>>());
        Reply { text: format!("To {} you need {need_list}.{notes}", self.task_phrase(b, &task)), intent: "answer", claims }
    }

    pub(crate) fn task_duration(&mut self, b: &mut Brain, t: &str) -> Reply {
        let m = self.session_model(b);
        let task = self.resolve_task(&m, t).unwrap_or_else(|| t.to_string());
        let stated = m.facts_of("takes_time").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task.as_str()));
        if let Some(a) = stated {
            let unit = a.args[3].as_sym().unwrap_or("minute").to_string();
            let notes = self.constraint_notes(&m, &task);
            return Reply { text: format!("{} takes {}.{notes}", capitalize(&words(&task)), range(&a.args[1], &a.args[2], &unit)), intent: "answer", claims: vec![a] };
        }
        if let Some(a) = m.facts_of("total_minutes").into_iter().find(|a| a.args.first().and_then(|v| v.as_sym()) == Some(task.as_str())) {
            return Reply { text: format!("{} takes about {}.", capitalize(&words(&task)), range(&a.args[1], &a.args[2], "minute")), intent: "answer", claims: vec![a] };
        }
        Reply { text: format!("I don't know how long {} takes yet. How long does it usually take?", words(t)), intent: "teach_me", claims: Vec::new() }
    }

    pub(crate) fn task_next(&mut self, b: &mut Brain) -> Reply {
        let Some(task) = self.active_task.clone() else {
            return Reply { text: "Next in what? Tell me what you are working on.".into(), intent: "clarify", claims: Vec::new() };
        };
        let m = self.session_model(b);
        let next: Vec<String> = self.ordered_steps(&m, &task).into_iter().filter(|s| m.contains(&g("next_step", &[&task, s]))).collect();
        match next.first() {
            None => Reply { text: format!("That was the last step. {} is done!", capitalize(&words(&task))), intent: "answer", claims: Vec::new() },
            Some(s) => {
                let (text, mut claims) = self.step_text(&m, s);
                claims.push(g("next_step", &[&task, s]));
                self.last_step = Some(s.clone());
                Reply { text: format!("Next: {text}"), intent: "answer", claims }
            }
        }
    }




    pub(crate) fn feedback(&mut self, positive: bool) -> Reply {
        if positive {
            Reply { text: "Good, glad that helps.".into(), intent: "social", claims: Vec::new() }
        } else {
            Reply { text: "Sorry about that. What should I have said?".into(), intent: "clarify", claims: Vec::new() }
        }
    }

    /// A relation no registered predicate fits goes to the review queue.
    pub(crate) fn new_pred(&mut self, b: &mut Brain, atom: Atom, definition: String) -> Reply {
        let now = self.now();
        // Pending, not believed: the fact waits with the word in the queue.
        if let Some(g) = atom.to_ground() {
            let _ = crate::queue::enqueue(b, &g, &definition, &self.source);
        }
        b.kb.log_change(now, "queue", &format!("new_pred {atom}"), &definition);
        let name = atom.pred.replace('_', " ");
        Reply { text: format!("I don't know the relation \"{name}\" yet; I'll ask a teacher about it."), intent: "teach_me", claims: Vec::new() }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

#[cfg(test)]
mod similar_tests {
    use super::similar_task;

    #[test]
    fn differently_worded_tasks() {
        let known = ["boil_egg", "bake_potato", "garlic_tomato_pasta", "fry_egg_white"];
        let f = |t: &str| similar_task(t, known.iter().copied());
        assert_eq!(f("cook_egg").as_deref(), Some("boil_egg"));
        assert_eq!(f("cook_eggs").as_deref(), Some("boil_egg"));
        assert_eq!(f("make_potato").as_deref(), Some("bake_potato"));
        assert_eq!(f("make_garlic_tomato_pasta").as_deref(), Some("garlic_tomato_pasta"));
        assert_eq!(f("fry_egg"), None, "a different verb is a different task");
        assert_eq!(f("wash_car"), None);
    }
}

/// Replace a whole word or phrase (and its plural), fixing "a"/"an" before it.
pub(crate) fn swap_word(text: &str, from: &str, to: &str) -> String {
    let mut out = text.to_string();
    for (f, t) in [(sla_nlg::plural(from), sla_nlg::plural(to)), (from.to_string(), to.to_string())] {
        let an = |w: &str| if w.starts_with(|c: char| "aeiou".contains(c)) { "an" } else { "a" };
        out = out.replace(&format!(" {} {f}", an(&f)), &format!(" {} {t}", an(&t)));
        let mut res = String::new();
        let mut rest = out.as_str();
        while let Some(i) = rest.find(&f) {
            let before_ok = i == 0 || !rest[..i].ends_with(|c: char| c.is_alphanumeric());
            let after = &rest[i + f.len()..];
            let after_ok = !after.starts_with(|c: char| c.is_alphanumeric());
            res.push_str(&rest[..i]);
            res.push_str(if before_ok && after_ok { &t } else { &f });
            rest = after;
        }
        res.push_str(rest);
        out = res;
    }
    out
}

impl Agent {
    /// "boil an egg" / "make garlic tomato pasta" without a locked brain at hand.
    pub(crate) fn task_phrase_m(&self, task: &str) -> String {
        let first = task.split('_').next().unwrap_or("");
        let verb = self.translator.registry.preds.get(first).map_or(false, |p| p.verb) || matches!(first, "make" | "cook" | "bake" | "boil" | "plan" | "change" | "pack" | "write" | "clean" | "wash" | "fix" | "do");
        match (verb, task.split_once('_')) {
            (true, Some((v, rest))) => format!("{v} {}", with_article(rest)),
            (true, None) => words(task),
            (false, _) => format!("make {}", words(task)),
        }
    }
}
