//! Tokenizing, contractions, lemmas and plurals.

use crate::lexicon::VERBS;

#[derive(Debug, Clone, PartialEq)]
pub struct Tok {
    /// Lowercase word.
    pub w: String,
    /// Written with a capital first letter.
    pub cap: bool,
}

fn push(out: &mut Vec<Tok>, w: &str, cap: bool) {
    out.push(Tok { w: w.to_string(), cap });
}

/// Split a message into lowercase tokens with contractions expanded.
/// Commas are kept as tokens (if-then rules use them). Returns whether the
/// message is a question (ends with '?').
pub fn tokenize(text: &str) -> (Vec<Tok>, bool) {
    let text = text.replace(['\u{2019}', '\u{2018}', '`'], "'");
    let question = text.trim_end().ends_with('?') || text.contains('?');
    let mut out = Vec::new();
    for raw in text.split_whitespace() {
        // Separate a trailing comma; drop other punctuation at the edges.
        let has_comma = raw.trim_end_matches(['.', '!', '?', ';', ':', ')', '"']).ends_with(',');
        let word = raw.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-' && c != '_');
        let word = word.trim_matches('\'');
        if !word.is_empty() {
            let cap = word.chars().next().map_or(false, |c| c.is_uppercase());
            let lw = word.to_lowercase();
            expand(&lw, cap, &mut out);
        }
        if has_comma {
            push(&mut out, ",", false);
        }
    }
    (out, question)
}

fn expand(w: &str, cap: bool, out: &mut Vec<Tok>) {
    let neg = [
        ("doesn't", "does"),
        ("don't", "do"),
        ("didn't", "did"),
        ("isn't", "is"),
        ("aren't", "are"),
        ("wasn't", "was"),
        ("weren't", "were"),
        ("hasn't", "has"),
        ("haven't", "have"),
        ("can't", "can"),
        ("cannot", "can"),
        ("won't", "will"),
        ("wouldn't", "would"),
    ];
    for (c, aux) in neg {
        if w == c {
            push(out, aux, cap);
            push(out, "not", false);
            return;
        }
    }
    if let Some(stem) = w.strip_suffix("'m") {
        push(out, stem, cap);
        push(out, "am", false);
        return;
    }
    if let Some(stem) = w.strip_suffix("'re") {
        push(out, stem, cap);
        push(out, "are", false);
        return;
    }
    if let Some(stem) = w.strip_suffix("'ve") {
        push(out, stem, cap);
        push(out, "have", false);
        return;
    }
    if let Some(stem) = w.strip_suffix("'ll") {
        push(out, stem, cap);
        push(out, "will", false);
        return;
    }
    if let Some(stem) = w.strip_suffix("'d") {
        push(out, stem, cap);
        push(out, "would", false);
        return;
    }
    if let Some(stem) = w.strip_suffix("'s") {
        push(out, stem, cap);
        let is_word = matches!(stem, "that" | "what" | "it" | "he" | "she" | "who" | "where" | "there" | "here" | "how" | "everything" | "something" | "someone");
        push(out, if is_word { "is" } else { "'s" }, false);
        return;
    }
    if w.ends_with("s'") {
        // Plural possessive: "the dogs' food"
        push(out, w.trim_end_matches('\''), cap);
        push(out, "'s", false);
        return;
    }
    push(out, w, cap);
}

/// Base form of a verb: likes -> like, flies -> fly, watches -> watch, drinking -> drink.
pub fn lemma(w: &str) -> String {
    match w {
        "is" | "are" | "am" | "was" | "were" | "be" | "been" => return "be".into(),
        "has" | "had" => return "have".into(),
        "does" | "did" => return "do".into(),
        "goes" | "went" => return "go".into(),
        "ate" => return "eat".into(),
        "drank" => return "drink".into(),
        "made" => return "make".into(),
        _ => {}
    }
    if let Some(stem) = w.strip_suffix("ing") {
        if stem.len() >= 2 {
            if VERBS.contains(&stem) {
                return stem.to_string();
            }
            let with_e = format!("{stem}e");
            if VERBS.contains(&with_e.as_str()) {
                return with_e;
            }
            let b = stem.as_bytes();
            if b.len() >= 2 && b[b.len() - 1] == b[b.len() - 2] {
                let s = &stem[..stem.len() - 1];
                if VERBS.contains(&s) {
                    return s.to_string();
                }
            }
            if stem.ends_with('y') || VERBS.iter().any(|v| *v == stem) {
                return stem.to_string();
            }
        }
    }
    if VERBS.contains(&w) {
        return w.to_string();
    }
    if let Some(stem) = w.strip_suffix("ies") {
        if stem.len() >= 2 {
            return format!("{stem}y");
        }
    }
    for suf in ["ches", "shes", "sses", "xes", "zes", "oes"] {
        if w.ends_with(suf) {
            return w[..w.len() - 2].to_string();
        }
    }
    if w.len() > 2 && w.ends_with('s') && !w.ends_with("ss") {
        return w[..w.len() - 1].to_string();
    }
    w.to_string()
}

/// Singular form of a plural common noun.
pub fn singularize(w: &str) -> String {
    if w.len() <= 3 || !w.ends_with('s') || w.ends_with("ss") || w.ends_with("us") || w.ends_with("is") {
        return w.to_string();
    }
    if let Some(stem) = w.strip_suffix("ies") {
        return format!("{stem}y");
    }
    // -ache nouns: headaches -> headache (not "headach").
    if w.ends_with("aches") && (w.ends_with("headaches") || w == "aches" || w.ends_with("toothaches") || w.ends_with("stomachaches") || w.ends_with("caches") || w.ends_with("moustaches")) {
        return w[..w.len() - 1].to_string();
    }
    for suf in ["ches", "shes", "sses", "xes", "zes"] {
        if w.ends_with(suf) {
            return w[..w.len() - 2].to_string();
        }
    }
    if w.ends_with("oes") && w.len() > 5 {
        return w[..w.len() - 2].to_string();
    }
    w[..w.len() - 1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contractions() {
        let (t, q) = tokenize("Anna doesn't like Bob's dog, right?");
        let w: Vec<&str> = t.iter().map(|t| t.w.as_str()).collect();
        assert_eq!(w, vec!["anna", "does", "not", "like", "bob", "'s", "dog", ",", "right"]);
        assert!(q);
        assert!(t[0].cap && t[4].cap && !t[3].cap);
        let (t, _) = tokenize("That's wrong. I'm happy");
        let w: Vec<&str> = t.iter().map(|t| t.w.as_str()).collect();
        assert_eq!(w, vec!["that", "is", "wrong", "i", "am", "happy"]);
    }

    #[test]
    fn lemmas() {
        for (a, b) in [("likes", "like"), ("flies", "fly"), ("watches", "watch"), ("goes", "go"), ("drinking", "drink"), ("running", "run"), ("making", "make"), ("plays", "play"), ("like", "like")] {
            assert_eq!(lemma(a), b, "{a}");
        }
    }

    #[test]
    fn plurals() {
        for (a, b) in [("apples", "apple"), ("berries", "berry"), ("boxes", "box"), ("wings", "wing"), ("glass", "glass"), ("bus", "bus"), ("tomatoes", "tomato"), ("shoes", "shoe"), ("tea", "tea"), ("headaches", "headache"), ("beaches", "beach")] {
            assert_eq!(singularize(a), b, "{a}");
        }
    }
}
