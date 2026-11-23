//! Extraction of memories from conversations: the input, the prompt, and the
//! validation of the model's answer (docs/improve-memory.md §12.2 to §12.4;
//! tasks P3.2 and P3.3). Everything here is pure: the store runs the model and
//! saves what passes.

use super::model::{validate, MemoryCategory, MAX_CONTENT_CHARS};
use super::secret_scan;
use serde::Deserialize;
use uuid::Uuid;

/// Longest user message kept in the transcript, in characters.
const MAX_USER_CHARS: usize = 2_000;
/// Longest assistant answer kept in the transcript, in characters.
const MAX_ASSISTANT_CHARS: usize = 600;
/// Fenced code longer than this many lines is replaced by a marker.
const CODE_OMIT_LINES: usize = 20;
/// Longest candidate memory, in characters (§12.3: one standalone sentence).
const MAX_CANDIDATE_CHARS: usize = 300;

/// One message of the conversation, as extraction sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turn {
    /// "user" or "assistant". Tool output and reasoning never reach extraction.
    pub role: String,
    pub text: String,
}

/// The conversation as the model reads it: cut, with code omitted, and with
/// any line that looks like a secret removed (§12.2).
pub fn transcript(turns: &[Turn]) -> String {
    let mut out = String::new();
    for turn in turns {
        let limit = if turn.role == "user" {
            MAX_USER_CHARS
        } else {
            MAX_ASSISTANT_CHARS
        };
        let cleaned = redact_and_omit(&turn.text);
        let cut: String = cleaned.chars().take(limit).collect();
        out.push_str(&format!("{}: {}\n\n", turn.role, cut.trim()));
    }
    out
}

/// Replace long fenced code with a marker, and any secret-looking line with
/// `[redacted]`. Redaction here is for what the model sees; storage scans again.
fn redact_and_omit(text: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut code: Vec<&str> = Vec::new();
    let mut in_code = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            if in_code {
                if code.len() > CODE_OMIT_LINES {
                    out.push("[code omitted]".to_string());
                } else {
                    out.extend(code.drain(..).map(redact_line));
                }
                code.clear();
                in_code = false;
            } else {
                in_code = true;
            }
            continue;
        }
        if in_code {
            code.push(line);
        } else {
            out.push(redact_line(line));
        }
    }
    if in_code {
        out.extend(code.into_iter().map(redact_line));
    }
    out.join("\n")
}

fn redact_line(line: &str) -> String {
    if secret_scan::scan(line).is_some() {
        "[redacted]".to_string()
    } else {
        line.to_string()
    }
}

/// Whether to run extraction now (§12.1): enough new user messages, or the
/// session has ended with at least one new user message.
pub fn should_extract(new_user_messages: usize, every_n: usize, session_end: bool) -> bool {
    if new_user_messages == 0 {
        return false;
    }
    new_user_messages >= every_n || session_end
}

/// The extraction prompt (§12.3), with the saved memories that might be
/// updated and the transcript. At session end it also asks for one episode.
pub fn build_prompt(existing: &[(Uuid, String)], transcript: &str, session_end: bool) -> String {
    let saved = if existing.is_empty() {
        "(none)".to_string()
    } else {
        existing
            .iter()
            .map(|(id, text)| format!("- {id}: {text}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let episode = if session_end {
        "\nAlso return exactly one memory with category \"episode\", scope \"project\" and\n\
         evidence \"user\": in one or two sentences, what was worked on in this session,\n\
         where it stopped, and what is left to do.\n"
    } else {
        ""
    };
    format!(
        "You extract long-term memories for Crustly, a coding assistant.\n\n\
         You get recent messages from one conversation and some memories that are already\n\
         saved. Return the facts worth remembering in future, separate conversations.\n\n\
         Save only what is:\n\
         - stated by the user, or proposed by the assistant and clearly accepted by the user;\n\
         - likely to stay true for weeks;\n\
         - useful later: the user's preferences, project conventions, decisions (with what\n\
           was rejected), facts about the project, lessons learned, and corrections to\n\
           memories that are already saved.\n\n\
         Do not save:\n\
         - requests that only concern this conversation, greetings, questions;\n\
         - anything the assistant suggested that the user did not accept;\n\
         - code, command output, file contents, stack traces;\n\
         - passwords, keys, tokens or other secrets;\n\
         - anything already in the saved memories, unless it changes one: then set\n\
           \"replaces\" to that memory's id.\n\n\
         Each memory is one standalone sentence of at most {MAX_CANDIDATE_CHARS} characters that makes\n\
         sense without the conversation. Keep its conditions. Do not turn uncertainty into certainty.\n\n\
         scope: \"global\" for facts about the user that hold in any project; otherwise \"project\".\n\
         category: preference | convention | decision | fact | lesson\n\
         importance: 1 (minor) to 5 (critical)\n\
         evidence: \"user\" (the user said it) or \"confirmed\" (the user accepted the assistant's proposal)\n\
         expires_in_days: null, or 1 to 365 for a fact that stops being true at a known time\n\
         {episode}\n\
         Return only JSON:\n\
         {{\"memories\": [{{\"content\": \"...\", \"scope\": \"project\", \"category\": \"convention\",\n\
           \"importance\": 4, \"evidence\": \"user\", \"replaces\": null, \"expires_in_days\": null}}]}}\n\
         Return {{\"memories\": []}} when nothing qualifies.\n\n\
         SAVED MEMORIES:\n{saved}\n\n\
         MESSAGES:\n{transcript}"
    )
}

/// One memory the model proposed, after validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub content: String,
    pub global: bool,
    pub category: MemoryCategory,
    pub importance: u8,
    /// True when the user said it (or the assistant's proposal was accepted).
    pub evidence_user: bool,
    pub replaces: Option<Uuid>,
    pub expires_in_days: Option<i64>,
    pub episode: bool,
}

/// A candidate as the model wrote it, before checks. Missing fields fail to
/// parse, and that candidate alone is dropped.
#[derive(Deserialize)]
struct Raw {
    content: String,
    scope: String,
    category: String,
    importance: i64,
    evidence: String,
    #[serde(default)]
    replaces: Option<String>,
    #[serde(default)]
    expires_in_days: Option<i64>,
}

/// Why the answer could not be used at all (the whole run then counts as failed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    NoJson,
    NotAnObject,
}

/// The first JSON object in the answer, with its candidates validated.
/// Invalid candidates are dropped, never repaired. An episode is kept only at
/// session end, and only the first one. At most `max_per_run` other candidates.
pub fn parse(
    text: &str,
    session_end: bool,
    min_importance: u8,
    max_per_run: usize,
) -> Result<Vec<Candidate>, ParseError> {
    let start = text.find('{').ok_or(ParseError::NoJson)?;
    let end = text.rfind('}').ok_or(ParseError::NoJson)?;
    if end < start {
        return Err(ParseError::NoJson);
    }
    let value: serde_json::Value =
        serde_json::from_str(&text[start..=end]).map_err(|_| ParseError::NoJson)?;
    let items = value
        .get("memories")
        .and_then(|m| m.as_array())
        .ok_or(ParseError::NotAnObject)?;

    let mut kept = Vec::new();
    let mut episodes = 0;
    let mut others = 0;
    for item in items {
        let Ok(raw) = serde_json::from_value::<Raw>(item.clone()) else {
            continue;
        };
        let Some(candidate) = check(raw, session_end, min_importance) else {
            continue;
        };
        if candidate.episode {
            if episodes == 0 {
                kept.push(candidate);
            }
            episodes += 1;
        } else if others < max_per_run {
            kept.push(candidate);
            others += 1;
        }
    }
    Ok(kept)
}

fn check(raw: Raw, session_end: bool, min_importance: u8) -> Option<Candidate> {
    let content = raw.content.trim().to_string();
    if content.is_empty() || content.chars().count() > MAX_CANDIDATE_CHARS.min(MAX_CONTENT_CHARS) {
        return None;
    }
    if validate(&content).is_err() {
        return None;
    }
    let global = match raw.scope.as_str() {
        "global" => true,
        "project" => false,
        _ => return None,
    };
    let category = raw.category.parse::<MemoryCategory>().ok()?;
    let episode = category == MemoryCategory::Episode;
    if episode && !session_end {
        return None;
    }
    if episode && global {
        return None;
    }
    let importance = u8::try_from(raw.importance)
        .ok()
        .filter(|i| (1..=5).contains(i))?;
    if importance < min_importance {
        return None;
    }
    let evidence_user = match raw.evidence.as_str() {
        "user" => true,
        "confirmed" => false,
        _ => return None,
    };
    let replaces = match raw.replaces.as_deref() {
        None | Some("") => None,
        Some(text) => Some(Uuid::parse_str(text).ok()?),
    };
    if let Some(days) = raw.expires_in_days {
        if !(1..=365).contains(&days) {
            return None;
        }
    }
    Some(Candidate {
        content,
        global,
        category,
        importance,
        evidence_user,
        replaces,
        expires_in_days: raw.expires_in_days,
        episode,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn turn(role: &str, text: &str) -> Turn {
        Turn {
            role: role.into(),
            text: text.into(),
        }
    }

    #[test]
    fn long_user_messages_are_cut_and_answers_cut_shorter() {
        let long = "a".repeat(3_000);
        let answer = "b".repeat(3_000);
        let t = transcript(&[turn("user", &long), turn("assistant", &answer)]);
        assert!(
            !t.contains(&"a".repeat(MAX_USER_CHARS + 1)),
            "the user message is cut"
        );
        assert!(
            !t.contains(&"b".repeat(MAX_ASSISTANT_CHARS + 1)),
            "the answer is cut shorter"
        );
        assert!(t.contains("user: "));
    }

    #[test]
    fn long_code_is_omitted_and_short_code_is_kept() {
        let long_code: String = (0..30).map(|i| format!("let x{i} = {i};\n")).collect();
        let text = format!("look:\n```rust\n{long_code}```\nand:\n```rust\nlet y = 1;\n```");
        let t = transcript(&[turn("user", &text)]);
        assert!(t.contains("[code omitted]"));
        assert!(t.contains("let y = 1;"));
        assert!(!t.contains("let x5"));
    }

    #[test]
    fn secret_lines_are_redacted_before_the_model_sees_them() {
        let key = ["AKIA", "ABCDEFGHIJKLMNOP"].concat();
        let t = transcript(&[turn("user", &format!("my key is {key}\nplease keep going"))]);
        assert!(!t.contains(&key));
        assert!(t.contains("[redacted]") && t.contains("please keep going"));
    }

    #[test]
    fn extraction_runs_after_enough_messages_or_at_session_end() {
        assert!(!should_extract(0, 6, true), "nothing new, nothing to do");
        assert!(!should_extract(5, 6, false));
        assert!(should_extract(6, 6, false));
        assert!(should_extract(1, 6, true));
    }

    #[test]
    fn the_prompt_carries_the_saved_memories_and_the_transcript() {
        let id = Uuid::new_v4();
        let p = build_prompt(&[(id, "Uses PostgreSQL".into())], "user: hello\n\n", false);
        assert!(p.contains(&id.to_string()) && p.contains("Uses PostgreSQL"));
        assert!(p.contains("user: hello"));
        assert!(!p.contains("\"episode\""), "no episode outside session end");
        assert!(build_prompt(&[], "", true).contains("category \"episode\""));
    }

    #[test]
    fn the_spec_example_parses() {
        let answer = r#"{"memories": [
            {"content": "The user prefers Tokio for asynchronous Rust projects.",
             "scope": "global", "category": "preference", "importance": 4,
             "evidence": "user", "replaces": null, "expires_in_days": null},
            {"content": "Avoid adding dependencies to the user's Rust projects unless necessary.",
             "scope": "global", "category": "convention", "importance": 4,
             "evidence": "user", "replaces": null, "expires_in_days": null}]}"#;
        let found = parse(answer, false, 3, 5).unwrap();
        assert_eq!(found.len(), 2);
        assert!(found[0].global && found[0].evidence_user);
    }

    #[test]
    fn invalid_candidates_are_dropped_one_by_one() {
        let answer = r#"{"memories": [
            {"content": "Good fact.", "scope": "project", "category": "fact", "importance": 3, "evidence": "user"},
            {"content": "Bad scope.", "scope": "team", "category": "fact", "importance": 3, "evidence": "user"},
            {"content": "Bad category.", "scope": "project", "category": "gossip", "importance": 3, "evidence": "user"},
            {"content": "Too unimportant.", "scope": "project", "category": "fact", "importance": 1, "evidence": "user"},
            {"content": "Bad evidence.", "scope": "project", "category": "fact", "importance": 3, "evidence": "guess"},
            {"content": "Bad expiry.", "scope": "project", "category": "fact", "importance": 3, "evidence": "user", "expires_in_days": 9999},
            {"scope": "project", "category": "fact", "importance": 3, "evidence": "user"},
            {"content": "Bad id.", "scope": "project", "category": "fact", "importance": 3, "evidence": "user", "replaces": "not-an-id"}
        ]}"#;
        let found = parse(answer, false, 3, 5).unwrap();
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].content, "Good fact.");
    }

    #[test]
    fn an_episode_only_at_session_end_and_only_one() {
        let answer = r#"{"memories": [
            {"content": "Worked on the export.", "scope": "project", "category": "episode", "importance": 3, "evidence": "user"},
            {"content": "Also worked on tests.", "scope": "project", "category": "episode", "importance": 3, "evidence": "user"}]}"#;
        assert!(parse(answer, false, 3, 5).unwrap().is_empty());
        let at_end = parse(answer, true, 3, 5).unwrap();
        assert_eq!(at_end.len(), 1);
        assert!(at_end[0].episode);
    }

    #[test]
    fn the_per_run_limit_applies_to_normal_memories() {
        let items: String = (0..8)
            .map(|i| format!(r#"{{"content": "Fact {i}.", "scope": "project", "category": "fact", "importance": 3, "evidence": "user"}},"#))
            .collect();
        let answer = format!(r#"{{"memories": [{}]}}"#, items.trim_end_matches(','));
        assert_eq!(parse(&answer, false, 3, 5).unwrap().len(), 5);
    }

    #[test]
    fn an_answer_without_json_is_a_failure() {
        assert_eq!(
            parse("I could not find anything.", false, 3, 5),
            Err(ParseError::NoJson)
        );
        assert_eq!(
            parse(r#"{"other": 1}"#, false, 3, 5),
            Err(ParseError::NotAnObject)
        );
    }
}
