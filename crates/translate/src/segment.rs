//! Stage 1-2 of understanding: split a message into parts without a model.
//!
//! - Lines and list items are their own parts: `- item`, `* item`, `1. item`,
//!   `1) item`, also when a list is written inline ("1) Boil ... 2) Warm ...",
//!   "- Sunscreen - Water - Hat").
//! - Sentences end at `.`, `!`, `?` followed by a space or the end, but not
//!   inside a decimal ("2.5"), after an abbreviation ("e.g.", "approx.",
//!   "Dr.") or after a single capital (an initial).
//! - v1's rule that turned ", and" into a sentence break is gone: a sentence
//!   stays whole, and is cut at clause words only when it fails (see
//!   [`clauses`]).

/// Abbreviations that do not end a sentence (lowercase, without the final dot).
const ABBREVIATIONS: &[&str] = &[
    "e.g", "i.e", "etc", "approx", "vs", "mr", "mrs", "ms", "dr", "prof", "st", "no", "min", "max", "hr", "hrs", "sec", "tbsp", "tsp", "oz", "lb", "lbs", "cf", "fig", "a.m", "p.m", "jr", "sr", "dept", "est",
];

/// Split a message into parts (sentences and list items), in order.
pub fn segment(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        for item in list_items(line) {
            out.extend(sentences(&item));
        }
    }
    out.into_iter().filter(|s| s.chars().any(|c| c.is_alphanumeric())).collect()
}

/// A line that is (or contains) a list becomes one part per item; the text
/// before the first item is a part of its own.
fn list_items(line: &str) -> Vec<String> {
    // A whole line that starts with a bullet or a number is one item.
    let starts: Vec<usize> = item_starts(line);
    if starts.is_empty() {
        return vec![line.to_string()];
    }
    let mut out = Vec::new();
    if starts[0] > 0 {
        out.push(line[..starts[0]].trim().to_string());
    }
    for (k, &s) in starts.iter().enumerate() {
        let end = starts.get(k + 1).copied().unwrap_or(line.len());
        let item = strip_marker(line[s..end].trim());
        if !item.is_empty() {
            out.push(item.to_string());
        }
    }
    out
}

/// Byte positions where list items start: a bullet `- ` / `* ` at the line
/// start or after a space when the line has two or more, or `N.` / `N)`
/// markers counting 1, 2, 3...
fn item_starts(line: &str) -> Vec<usize> {
    let b = line.as_bytes();
    // Numbered: "1) ", "2. " in sequence.
    let mut numbered = Vec::new();
    let mut want = 1;
    let mut i = 0;
    while i < b.len() {
        let at_word = i == 0 || b[i - 1] == b' ';
        if at_word && b[i].is_ascii_digit() {
            let mut j = i;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            if j + 1 < b.len() && (b[j] == b')' || b[j] == b'.') && b[j + 1] == b' ' {
                if line[i..j].parse::<usize>().ok() == Some(want) {
                    numbered.push(i);
                    want += 1;
                }
            }
            i = j;
            continue;
        }
        i += 1;
    }
    if numbered.len() >= 2 || (numbered.len() == 1 && numbered[0] == 0) {
        return numbered;
    }
    // Bullets.
    let mut bullets = Vec::new();
    for (i, w) in b.windows(2).enumerate() {
        if (w == b"- " || w == b"* ") && (i == 0 || b[i - 1] == b' ') {
            bullets.push(i);
        }
    }
    if bullets.len() >= 2 || (bullets.len() == 1 && bullets[0] == 0) {
        return bullets;
    }
    Vec::new()
}

fn strip_marker(s: &str) -> &str {
    let s = s.trim_start_matches(['-', '*']).trim_start();
    let digits = s.bytes().take_while(|c| c.is_ascii_digit()).count();
    if digits > 0 && s[digits..].starts_with([')', '.']) {
        return s[digits + 1..].trim_start();
    }
    s
}

/// Split one line into sentences.
pub fn sentences(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        cur.push(c);
        if matches!(c, '.' | '!' | '?') {
            // Swallow repeated end marks and closing quotes ("Really?!", "...").
            while i + 1 < chars.len() && matches!(chars[i + 1], '.' | '!' | '?' | '"' | '\'' | ')') {
                i += 1;
                cur.push(chars[i]);
            }
            let next_is_space = chars.get(i + 1).map_or(true, |n| n.is_whitespace());
            if next_is_space && !(c == '.' && keeps_going(&cur, &chars, i)) {
                out.push(cur.trim().to_string());
                cur.clear();
            }
        }
        i += 1;
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

/// A dot that does not end the sentence: after an abbreviation or an
/// initial, or when the next word starts lowercase after an abbreviation-like word.
fn keeps_going(cur: &str, chars: &[char], i: usize) -> bool {
    let body = cur.trim_end_matches(['.', '"', '\'', ')']);
    let last = body.rsplit(|c: char| c.is_whitespace() || c == '(').next().unwrap_or("");
    let low = last.to_lowercase();
    if ABBREVIATIONS.contains(&low.as_str()) {
        return true;
    }
    // A single capital letter: an initial ("J. Smith").
    if last.len() == 1 && last.chars().all(|c| c.is_ascii_uppercase()) {
        return true;
    }
    // "e.g" style with inner dots.
    if low.contains('.') && low.len() <= 5 && low.chars().all(|c| c.is_ascii_alphabetic() || c == '.') {
        return true;
    }
    let _ = (chars, i);
    false
}

/// Clauses of a sentence that failed as a whole, cut at clause words
/// ("and", "but", "then", "while", "because", "so", ";"). Each clause keeps
/// the word that links it, so the links can still be read.
pub fn clauses(sentence: &str) -> Vec<String> {
    let s = sentence.trim().trim_end_matches(['.', '!', '?']);
    let mut out: Vec<String> = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    const CUT: &[&str] = &["and", "but", "then", "while", "because", "so", "meanwhile", "afterwards", "after"];
    for w in s.split_whitespace() {
        let bare = w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
        let after_comma = cur.last().map_or(false, |p| p.ends_with([',', ';']));
        if CUT.contains(&bare.as_str()) && cur.len() >= 2 && (after_comma || bare != "and") {
            out.push(cur.join(" ").trim_end_matches([',', ';']).to_string());
            cur.clear();
        } else if w.ends_with(';') && !cur.is_empty() {
            cur.push(w.trim_end_matches(';'));
            out.push(cur.join(" "));
            cur.clear();
            continue;
        }
        cur.push(w);
    }
    if !cur.is_empty() {
        out.push(cur.join(" "));
    }
    out.into_iter().filter(|c| c.split_whitespace().count() >= 2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sentences_and_lists() {
        assert_eq!(segment("Hi! Can you help me plan dinner tonight?"), vec!["Hi!", "Can you help me plan dinner tonight?"]);
        assert_eq!(segment("It costs 2.5 euros. Fine."), vec!["It costs 2.5 euros.", "Fine."]);
        assert_eq!(segment("Use herbs, e.g. basil or mint. Then water them."), vec!["Use herbs, e.g. basil or mint.", "Then water them."]);
        assert_eq!(segment("Wait approx. 10 minutes. Done."), vec!["Wait approx. 10 minutes.", "Done."]);
        assert_eq!(segment("I have pasta, garlic, and tomatoes."), vec!["I have pasta, garlic, and tomatoes."], "', and' is not a sentence break");
        assert_eq!(
            segment("Steps: 1) Boil the pasta. 2) Warm the garlic. 3) Mix."),
            vec!["Steps:", "Boil the pasta.", "Warm the garlic.", "Mix."]
        );
        assert_eq!(segment("Pack this: - Sunscreen - Water bottle - Hat"), vec!["Pack this:", "Sunscreen", "Water bottle", "Hat"]);
        assert_eq!(segment("- eggs\n- milk\n\nThanks!"), vec!["eggs", "milk", "Thanks!"]);
        assert_eq!(segment("Really?! Wow..."), vec!["Really?!", "Wow..."]);
        assert_eq!(segment("Mr. Smith is here. Ok"), vec!["Mr. Smith is here.", "Ok"]);
        assert!(segment("  ").is_empty());
    }

    #[test]
    fn clause_cuts() {
        assert_eq!(clauses("Boil the pasta, and while it cooks warm the garlic."), vec!["Boil the pasta", "and while it cooks warm the garlic"]);
        assert_eq!(clauses("I have pasta and garlic."), vec!["I have pasta and garlic"], "a plain 'and' list is not cut");
        assert_eq!(clauses("Fry the garlic, then add the tomatoes because they need time."), vec!["Fry the garlic", "then add the tomatoes", "because they need time"]);
    }
}
