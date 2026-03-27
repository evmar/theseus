//! winapi call tracing support.
//!
//! dllexport functions generate code to look up their tracing status from here.

use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub enum Trace {
    Unknown,
    None,
    Before,
}

struct Rule {
    key: String,
    enabled: bool,
}

struct State {
    rules: Vec<Rule>,
}

impl State {
    fn parse(scheme: &str) -> Self {
        let mut rules = vec![];
        for mut part in scheme.split(',') {
            let mut enabled = true;
            if part.is_empty() {
                continue;
            } else if part.starts_with('-') {
                part = &part[1..];
                enabled = false;
            } else if part.starts_with('+') {
                part = &part[1..];
            }
            rules.push(Rule {
                key: part.into(),
                enabled,
            });
        }
        State { rules }
    }
}

static STATE: OnceLock<State> = OnceLock::new();

pub fn init(scheme: &str) {
    STATE.get_or_init(|| State::parse(scheme));
}

pub fn get(cache: &mut Trace, path: &str) -> Trace {
    if !matches!(*cache, Trace::Unknown) {
        return *cache;
    }
    let state = STATE.get().unwrap();
    let mut enabled = false;
    for rule in &state.rules {
        if path.contains(&rule.key) {
            enabled = rule.enabled;
            // Don't break, let last match win.
        }
    }
    let trace = if enabled { Trace::Before } else { Trace::None };
    *cache = trace;
    trace
}
