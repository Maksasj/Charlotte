//! Markdown documents with fenced ```datalog blocks.
//!
//! Text outside the blocks is kept verbatim. Inside a block each line is a
//! fact or rule (depending on the file kind), a comment/blank line, or an
//! invalid line that is kept as written and reported. Unmodified lines are
//! written back byte-for-byte, so parse + write never changes a file.

use crate::syntax::{self, ParseError};
use crate::term::{Fact, Rule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    /// Lines are facts with `@` annotations.
    Facts,
    /// Lines are rules (a ground statement is a rule with an empty body).
    Rules,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Fact(Fact),
    Rule(Rule),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    /// A parsed statement. `raw` holds the original text while unmodified.
    Item { item: Item, raw: Option<String> },
    /// Blank line or `%` comment.
    Other(String),
    /// Unparseable line, preserved as written.
    Invalid { raw: String, error: String },
}

impl Line {
    fn render(&self) -> String {
        match self {
            Line::Item { raw: Some(r), .. } => r.clone(),
            Line::Item { item: Item::Fact(f), raw: None } => f.to_line(),
            Line::Item { item: Item::Rule(r), raw: None } => r.to_string(),
            Line::Other(s) => s.clone(),
            Line::Invalid { raw, .. } => raw.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The opening fence line as written, e.g. "```datalog".
    pub open: String,
    /// The closing fence line as written.
    pub close: String,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Text(String),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub segments: Vec<Segment>,
    pub kind: BlockKind,
    /// "\n" or "\r\n", detected from the input.
    pub newline: String,
    /// The input ended with a newline.
    pub trailing_newline: bool,
}

fn is_open_fence(line: &str) -> bool {
    let t = line.trim();
    t.starts_with("```") && t[3..].trim() == "datalog"
}

fn is_close_fence(line: &str) -> bool {
    line.trim() == "```"
}

fn parse_line(text: &str, kind: BlockKind) -> Line {
    let t = text.trim();
    if t.is_empty() || t.starts_with('%') {
        return Line::Other(text.to_string());
    }
    let res: Result<Item, ParseError> = match kind {
        BlockKind::Facts => syntax::parse_fact_line(t).map(Item::Fact),
        BlockKind::Rules => syntax::parse_rule(t).map(Item::Rule),
    };
    match res {
        Ok(item) => Line::Item { item, raw: Some(text.to_string()) },
        Err(e) => Line::Invalid { raw: text.to_string(), error: e.0 },
    }
}

impl Document {
    pub fn new(kind: BlockKind) -> Self {
        Document { segments: Vec::new(), kind, newline: "\n".into(), trailing_newline: true }
    }

    pub fn parse(src: &str, kind: BlockKind) -> Self {
        let newline = if src.contains("\r\n") { "\r\n" } else { "\n" }.to_string();
        let trailing_newline = src.ends_with('\n');
        let body = src.strip_suffix('\n').unwrap_or(src);
        let body = body.strip_suffix('\r').filter(|_| newline == "\r\n").unwrap_or(body);
        let mut segments = Vec::new();
        let mut text: Vec<&str> = Vec::new();
        let mut block: Option<Block> = None;
        if !src.is_empty() {
            for line in body.split(newline.as_str()) {
                match block.as_mut() {
                    None => {
                        if is_open_fence(line) {
                            if !text.is_empty() {
                                segments.push(Segment::Text(text.join(&newline)));
                                text.clear();
                            }
                            block = Some(Block { open: line.to_string(), close: String::new(), lines: Vec::new() });
                        } else {
                            text.push(line);
                        }
                    }
                    Some(b) => {
                        if is_close_fence(line) {
                            b.close = line.to_string();
                            segments.push(Segment::Block(block.take().unwrap()));
                        } else {
                            b.lines.push(parse_line(line, kind));
                        }
                    }
                }
            }
        }
        if let Some(mut b) = block {
            // Unclosed block: keep it, and close it on write.
            b.close = "```".into();
            segments.push(Segment::Block(b));
        }
        if !text.is_empty() {
            segments.push(Segment::Text(text.join(&newline)));
        }
        Document { segments, kind, newline, trailing_newline }
    }

    pub fn render(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        for seg in &self.segments {
            match seg {
                Segment::Text(t) => lines.push(t.clone()),
                Segment::Block(b) => {
                    lines.push(b.open.clone());
                    for l in &b.lines {
                        lines.push(l.render());
                    }
                    lines.push(b.close.clone());
                }
            }
        }
        let mut s = lines.join(&self.newline);
        if self.trailing_newline && !s.is_empty() {
            s.push_str(&self.newline);
        }
        s
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.segments.iter().filter_map(|s| match s {
            Segment::Block(b) => Some(b),
            Segment::Text(_) => None,
        })
    }

    pub fn lines_mut(&mut self) -> impl Iterator<Item = &mut Line> {
        self.segments.iter_mut().flat_map(|s| match s {
            Segment::Block(b) => b.lines.iter_mut().collect::<Vec<_>>(),
            Segment::Text(_) => Vec::new(),
        })
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.blocks().flat_map(|b| b.lines.iter()).filter_map(|l| match l {
            Line::Item { item, .. } => Some(item),
            _ => None,
        })
    }

    pub fn facts(&self) -> impl Iterator<Item = &Fact> {
        self.items().filter_map(|i| match i {
            Item::Fact(f) => Some(f),
            Item::Rule(_) => None,
        })
    }

    pub fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.items().filter_map(|i| match i {
            Item::Rule(r) => Some(r),
            Item::Fact(_) => None,
        })
    }

    /// Invalid lines with their errors, for reporting to the user.
    pub fn errors(&self) -> Vec<(String, String)> {
        self.blocks()
            .flat_map(|b| b.lines.iter())
            .filter_map(|l| match l {
                Line::Invalid { raw, error } => Some((raw.clone(), error.clone())),
                _ => None,
            })
            .collect()
    }

    fn last_block_mut(&mut self) -> &mut Block {
        let has_block = self.segments.iter().any(|s| matches!(s, Segment::Block(_)));
        if !has_block {
            if !self.segments.is_empty() {
                self.segments.push(Segment::Text(String::new()));
            }
            self.segments.push(Segment::Block(Block {
                open: "```datalog".into(),
                close: "```".into(),
                lines: Vec::new(),
            }));
        }
        for s in self.segments.iter_mut().rev() {
            if let Segment::Block(b) = s {
                return b;
            }
        }
        unreachable!()
    }

    /// Append an item to the last datalog block (creating one if needed).
    pub fn push(&mut self, item: Item) {
        self.last_block_mut().lines.push(Line::Item { item, raw: None });
    }

    /// Change every item for which `f` returns true; changed lines are re-rendered.
    pub fn update_items(&mut self, mut f: impl FnMut(&mut Item) -> bool) -> usize {
        let mut n = 0;
        for line in self.lines_mut() {
            if let Line::Item { item, raw } = line {
                if f(item) {
                    *raw = None;
                    n += 1;
                }
            }
        }
        n
    }

    /// Remove items for which `f` returns true. Used only for rules
    /// (facts are never deleted, only closed).
    pub fn remove_items(&mut self, mut f: impl FnMut(&Item) -> bool) -> usize {
        let mut n = 0;
        for seg in &mut self.segments {
            if let Segment::Block(b) = seg {
                let before = b.lines.len();
                b.lines.retain(|l| !matches!(l, Line::Item { item, .. } if f(item)));
                n += before - b.lines.len();
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# maksim\n\nNotes about maksim.\n\n```datalog\n% food\nlikes(maksim, coffee). @from 2026-11-16 @conf 0.9 @source user\nlikes(maksim,tea).   @conf 1 @source user\nbroken(\n```\n\nMore text.\n";

    #[test]
    fn lossless() {
        let d = Document::parse(SAMPLE, BlockKind::Facts);
        assert_eq!(d.render(), SAMPLE);
        assert_eq!(d.facts().count(), 2);
        assert_eq!(d.errors().len(), 1);
    }

    #[test]
    fn crlf_lossless() {
        let s = SAMPLE.replace('\n', "\r\n");
        let d = Document::parse(&s, BlockKind::Facts);
        assert_eq!(d.render(), s);
    }

    #[test]
    fn no_trailing_newline() {
        let s = "text\n```datalog\np(a). @conf 1 @source x\n```";
        assert_eq!(Document::parse(s, BlockKind::Facts).render(), s);
    }

    #[test]
    fn modify_only_changes_one_line() {
        let mut d = Document::parse(SAMPLE, BlockKind::Facts);
        let n = d.update_items(|i| match i {
            Item::Fact(f) if f.atom.args[1].as_sym() == Some("tea") => {
                f.conf = 0.5;
                true
            }
            _ => false,
        });
        assert_eq!(n, 1);
        let out = d.render();
        let changed: Vec<_> = SAMPLE.lines().zip(out.lines()).filter(|(a, b)| a != b).collect();
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].1, "likes(maksim, tea). @conf 0.5 @source user");
    }

    #[test]
    fn push_creates_block() {
        let mut d = Document::parse("# bob\n", BlockKind::Rules);
        d.push(Item::Rule(syntax::parse_rule("p(X) :- q(X).").unwrap()));
        assert_eq!(d.render(), "# bob\n\n```datalog\np(X) :- q(X).\n```\n");
        let d2 = Document::parse(&d.render(), BlockKind::Rules);
        assert_eq!(d2.rules().count(), 1);
    }
}
