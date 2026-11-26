use super::VaultBuilder;
use super::syntax::{
    parse_action, parse_call, parse_condition, split_outside_quotes, split_top_level, unquote,
};
use crate::types::{Action, Condition, Episode, Fact, Pattern, PatternKind, PatternStatus, Scope};

#[derive(PartialEq)]
enum Section {
    None,
    When,
    Then,
}

pub(crate) struct ModuleParser<'a> {
    builder: &'a mut VaultBuilder,
    provenance: String,
    in_fence: bool,
    parse_fence: bool,
    current: Option<Pattern>,
    section: Section,
}

impl<'a> ModuleParser<'a> {
    pub fn new(builder: &'a mut VaultBuilder, module: &str) -> Self {
        Self {
            builder,
            provenance: format!("vault:{module}"),
            in_fence: false,
            parse_fence: false,
            current: None,
            section: Section::None,
        }
    }

    pub fn parse(mut self, raw: &str) -> Result<(), String> {
        for (idx, raw_line) in raw.lines().enumerate() {
            let line_no = idx + 1;
            let trimmed = raw_line.trim();

            if let Some(rest) = trimmed.strip_prefix("```") {
                self.toggle_fence(rest.trim());
                continue;
            }

            if !self.in_fence || !self.parse_fence || is_ignored_line(trimmed) {
                continue;
            }

            if let Some(name) = extract_rule_name(trimmed) {
                self.start_pattern(name);
                continue;
            }

            let is_indented = raw_line.starts_with(' ') || raw_line.starts_with('\t');
            if self.current.is_some()
                && self.try_parse_pattern_line(trimmed, is_indented, line_no)?
            {
                continue;
            }

            self.flush_pattern();
            self.parse_top_level(trimmed, line_no)?;
        }

        self.flush_pattern();
        Ok(())
    }

    fn toggle_fence(&mut self, lang: &str) {
        if !self.in_fence {
            self.in_fence = true;
            self.parse_fence = lang.is_empty() || lang.eq_ignore_ascii_case("karin");
        } else {
            self.flush_pattern();
            self.in_fence = false;
            self.parse_fence = false;
        }
    }

    fn start_pattern(&mut self, name: String) {
        self.flush_pattern();
        let id = self.builder.alloc_pattern_id();
        self.current = Some(Pattern {
            id,
            name,
            status: PatternStatus::Active,
            when: Vec::new(),
            then: Vec::new(),
            trust: 0.9,
            support: 3,
            provenance: self.provenance.clone(),
            kind: PatternKind::Reply,
        });
        self.section = Section::None;
    }

    fn flush_pattern(&mut self) {
        if let Some(mut pattern) = self.current.take() {
            infer_kind(&mut pattern);
            self.builder.patterns.push(pattern);
        }
        self.section = Section::None;
    }

    fn try_parse_pattern_line(
        &mut self,
        trimmed: &str,
        is_indented: bool,
        line_no: usize,
    ) -> Result<bool, String> {
        let Some(pattern) = self.current.as_mut() else {
            return Ok(false);
        };

        if let Some(rest) = trimmed.strip_prefix("when:") {
            self.section = Section::When;
            push_conditions(pattern, rest.trim(), line_no)?;
            return Ok(true);
        }
        if let Some(rest) = trimmed.strip_prefix("then:") {
            self.section = Section::Then;
            push_actions(pattern, rest.trim(), line_no)?;
            return Ok(true);
        }
        if let Some(rest) = trimmed.strip_prefix("status:") {
            self.section = Section::None;
            pattern.status = parse_status(rest.trim());
            return Ok(true);
        }
        if let Some(rest) = trimmed.strip_prefix("kind:") {
            self.section = Section::None;
            pattern.kind = parse_kind(rest.trim());
            return Ok(true);
        }
        if let Some(rest) = trimmed.strip_prefix("trust:") {
            self.section = Section::None;
            pattern.trust = rest
                .trim()
                .parse()
                .map_err(|_| format!("line {line_no}: invalid trust '{}'", rest.trim()))?;
            return Ok(true);
        }
        if let Some(rest) = trimmed.strip_prefix("support:") {
            self.section = Section::None;
            pattern.support = rest
                .trim()
                .parse()
                .map_err(|_| format!("line {line_no}: invalid support '{}'", rest.trim()))?;
            return Ok(true);
        }

        if (is_indented || trimmed.starts_with("- ")) && self.section != Section::None {
            let item = strip_bullet(trimmed);
            match self.section {
                Section::When => push_conditions(pattern, item, line_no)?,
                Section::Then => push_actions(pattern, item, line_no)?,
                Section::None => {}
            }
            return Ok(true);
        }

        Ok(false)
    }

    fn parse_top_level(&mut self, trimmed: &str, line_no: usize) -> Result<(), String> {
        let line = strip_bullet(trimmed);

        if line.starts_with('"')
            && let Some((lhs, rhs)) = split_outside_quotes(line, "=>")
        {
            let input = unquote(lhs.trim()).map_err(|err| format!("line {line_no}: {err}"))?;
            let reply = unquote(rhs.trim()).map_err(|err| format!("line {line_no}: {err}"))?;
            let id = self.builder.alloc_episode_id();
            self.builder.episodes.push(Episode {
                id,
                input,
                reply,
                source: self.provenance.clone(),
                generalized: false,
            });
            return Ok(());
        }

        let (scope, expr) = split_scope(line);
        if expr.contains('(') {
            let (rel, args) = parse_call(expr).map_err(|err| format!("line {line_no}: {err}"))?;
            let id = self.builder.alloc_fact_id();
            self.builder.facts.push(Fact {
                id,
                rel,
                args,
                scope,
                provenance: self.provenance.clone(),
                turn: 0,
            });
        }
        Ok(())
    }
}

fn is_ignored_line(trimmed: &str) -> bool {
    trimmed.is_empty()
        || trimmed.starts_with("//")
        || (trimmed.starts_with('#') && !trimmed.starts_with("## "))
}

fn extract_rule_name(trimmed: &str) -> Option<String> {
    trimmed
        .strip_prefix("rule ")
        .or_else(|| trimmed.strip_prefix("## "))
        .map(|name| name.trim().to_string())
}

fn strip_bullet(trimmed: &str) -> &str {
    trimmed.strip_prefix("- ").map(str::trim).unwrap_or(trimmed)
}

fn split_scope(line: &str) -> (Scope, &str) {
    if let Some(rest) = line.strip_prefix("[working]") {
        (Scope::Working, rest.trim())
    } else if let Some(rest) = line.strip_prefix("[chat]") {
        (Scope::Chat, rest.trim())
    } else if let Some(rest) = line.strip_prefix("[long_term]") {
        (Scope::LongTerm, rest.trim())
    } else {
        (Scope::LongTerm, line)
    }
}

fn push_conditions(pattern: &mut Pattern, text: &str, line_no: usize) -> Result<(), String> {
    if text.is_empty() {
        return Ok(());
    }
    for item in split_top_level(text, ',') {
        let cond = parse_condition(&item).map_err(|err| format!("line {line_no}: {err}"))?;
        pattern.when.push(cond);
    }
    Ok(())
}

fn push_actions(pattern: &mut Pattern, text: &str, line_no: usize) -> Result<(), String> {
    if text.is_empty() {
        return Ok(());
    }
    for item in split_top_level(text, ',') {
        let action = parse_action(&item).map_err(|err| format!("line {line_no}: {err}"))?;
        pattern.then.push(action);
    }
    Ok(())
}

fn parse_status(raw: &str) -> PatternStatus {
    match raw {
        "candidate" => PatternStatus::Candidate,
        _ => PatternStatus::Active,
    }
}

fn parse_kind(raw: &str) -> PatternKind {
    match raw {
        "language" => PatternKind::Language,
        "reasoning" => PatternKind::Reasoning,
        "composite" => PatternKind::Composite,
        "induced" => PatternKind::Induced,
        _ => PatternKind::Reply,
    }
}

fn infer_kind(pattern: &mut Pattern) {
    let has_reply = pattern
        .then
        .iter()
        .any(|action| matches!(action, Action::Reply { .. }));
    if has_reply {
        return;
    }

    let has_text = pattern
        .when
        .iter()
        .any(|cond| matches!(cond, Condition::Text { .. } | Condition::Contains { .. }));
    let has_tool = pattern
        .then
        .iter()
        .any(|action| matches!(action, Action::Tool { .. }));

    if has_text {
        pattern.kind = PatternKind::Language;
    } else if has_tool {
        pattern.kind = PatternKind::Reasoning;
    }
}
