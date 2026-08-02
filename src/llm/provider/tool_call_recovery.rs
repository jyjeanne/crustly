//! Shared tool-call recovery: detect a tool call a model printed as plain
//! JSON text instead of populating a provider's own native structured field.
//!
//! Originally written for `OllamaProvider` - some Ollama chat templates
//! (qwen2.5-coder's among them) never populate the native `tool_calls`
//! field; the model prints `{"name": ..., "arguments": {...}}` as message
//! content instead. Extracted here (`llama-cpp-2-integration-plan.md`
//! Phase 4) because `llama.cpp` has no native tool-calling API at all -
//! `LlamaCppProvider` needs exactly this same recovery, not a second,
//! divergent implementation of the same fairly intricate,
//! security-sensitive parsing.
//!
//! Deliberately strict throughout: a false positive would execute a tool
//! the model never asked for.

use super::types::Tool;

/// Whether the text so far could still turn out to be a tool call printed
/// as content, and so should be withheld from a live-streamed chat rather
/// than shown immediately.
///
/// Only a leading `{` (or a ```json fence) qualifies. Ordinary prose
/// therefore streams token-by-token as normal; the buffering cost falls
/// only on content that really does look like a call.
pub fn maybe_tool_call_json(text: &str) -> bool {
    let t = text.trim_start();
    t.is_empty() || t.starts_with('{') || t.starts_with("```")
}

/// Whether `text` (the *whole* response generated so far, from the very
/// first token - never a withheld/reset suffix) has committed to naming one
/// of `offered` via a `"name": "<tool>"` key.
///
/// Deliberately much stricter than [`maybe_tool_call_json`] - the two serve
/// opposite risk directions and must not be confused for each other:
/// `maybe_tool_call_json` decides what to *withhold* from a live stream, so
/// a false positive there only delays a harmless flush. This function
/// decides whether it's safe to switch to grammar-*constrained* decoding
/// (`llama-cpp-2-integration-plan.md` Phase 4b) - a false positive there
/// permanently forces whatever the model is actually generating (which
/// could be ordinary prose or an unrelated JSON answer that merely starts
/// with `{`) into a fabricated call to an offered tool, which then gets
/// executed. So this only returns `true` once the model has already typed
/// enough to unambiguously name a *real* offered tool, not merely opened a
/// brace.
///
/// Two narrowing choices, both safe-by-default (a `false` here just means
/// "keep decoding unconstrained, the always-on recovery heuristic still
/// gets a chance at the end" - never a hard failure):
/// - Only a *bare* leading `{` counts (mirrors `tool_call_from_content`'s
///   unfenced case) - a fenced block needs unconstrained prose around it
///   that a JSON-only grammar can't express, so that case is left entirely
///   to the recovery heuristic, same as before this function existed.
/// - The `"name": "<tool>"` match only recognizes the compact and
///   single-space-after-colon spellings a model's own generated JSON
///   typically uses (not arbitrary whitespace/newlines between key and
///   value) - missing an unusual spacing just means the constrained path
///   doesn't engage for that response, which is the safe direction to fail
///   in, not a correctness problem.
///
/// The match must be at brace depth 1 (directly inside the response's own
/// top-level object), not nested inside an array or sub-object - a
/// response like `{"records": [{"id": 1, "name": "bash"}], "summary": ...}`
/// must not trigger just because an unrelated, nested field happens to be
/// spelled `"name": "bash"`.
///
/// Only the first [`NAME_KEY_SCAN_WINDOW`] bytes are scanned - see its own
/// doc comment for why that's both correct and necessary for this to stay
/// cheap when called every token/chunk across a whole response, as
/// `run_complete`/`run_stream` do.
pub fn commits_to_an_offered_tool_call(text: &str, offered: &[Tool]) -> bool {
    let trimmed = text.trim_start();
    if !trimmed.starts_with('{') {
        return false;
    }
    let window = leading_window(text, NAME_KEY_SCAN_WINDOW);
    offered
        .iter()
        .any(|tool| top_level_name_key_matches(window, &tool.name))
}

/// How much of the response text [`commits_to_an_offered_tool_call`] scans.
/// A genuine top-level `"name"` key (the shape it looks for) appears within
/// the first few dozen bytes of any response that opens with it -
/// `tool_instructions_block` instructs the model to answer with *only*
/// `{"name": ..., "arguments": {...}}`, so `"name"` is always the very
/// first key of a real call. Bounding the scan to a generous fixed prefix,
/// instead of the whole ever-growing response, keeps the check O(1) per
/// call regardless of response length - re-scanning the *entire*
/// accumulated text from scratch on every token, for the common case of a
/// response that opens with `{` but never turns out to be a tool call
/// (tools are offered on nearly every turn; most responses aren't calls),
/// was an O(n²) cost over the response length.
const NAME_KEY_SCAN_WINDOW: usize = 256;

/// The first `max_bytes` of `text`, shortened to the nearest earlier UTF-8
/// character boundary so the slice never panics.
fn leading_window(text: &str, max_bytes: usize) -> &str {
    let mut end = text.len().min(max_bytes);
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}

/// Whether `window` contains `"name":"<tool_name>"` (or the
/// single-space-after-colon spelling) as a complete string value at brace
/// depth 1. Zero-allocation - no `format!`, just slice comparisons - since
/// this runs on a hot per-token path.
fn top_level_name_key_matches(window: &str, tool_name: &str) -> bool {
    window.match_indices(tool_name).any(|(idx, _)| {
        name_key_immediately_precedes(window, idx)
            && window[idx + tool_name.len()..].starts_with('"')
            && brace_depth_at(window, idx) == 1
    })
}

/// Whether `text[..idx]` ends with `"name":"` or `"name": "` - i.e. `idx`
/// is the start of a `"name"` key's string *value*, not merely a
/// coincidental occurrence of `tool_name` somewhere else in the text.
fn name_key_immediately_precedes(text: &str, idx: usize) -> bool {
    let prefix = &text[..idx];
    prefix.ends_with("\"name\":\"") || prefix.ends_with("\"name\": \"")
}

/// The `{`/`}` nesting depth of `text` at `byte_pos`, ignoring braces
/// inside string literals (respecting `\"` escapes). Used to tell a
/// response's own top-level `"name"` key (depth 1) apart from an
/// unrelated `"name"` field nested inside an array or sub-object (depth
/// 2+) - deliberately not tracking `[`/`]`, since the grammar this gates
/// already requires the top-level value to be an object, confirmed by the
/// caller's own leading-`{` check.
fn brace_depth_at(text: &str, byte_pos: usize) -> i32 {
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;
    for (i, c) in text.char_indices() {
        if i >= byte_pos {
            break;
        }
        if escaped {
            escaped = false;
            continue;
        }
        match c {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => depth -= 1,
            _ => {}
        }
    }
    depth
}

/// Recover a tool call that the model printed as text instead of returning
/// it via a provider's native structured tool-call mechanism (Ollama's
/// `tool_calls` field, or - for `llama.cpp`, which has no such mechanism at
/// all - the only way a call is ever recognized).
///
/// Deliberately strict, because a false positive would execute a tool the
/// model never asked for. The *entire* content must be one JSON object
/// carrying exactly a string `name` and an object `arguments` (or
/// `parameters`), and the name must match a tool that was actually offered.
/// Prose that merely contains JSON, a fenced example, or a call to an
/// unknown tool are all left as text.
pub fn tool_call_from_content(
    content: &str,
    offered: &[Tool],
) -> Option<(String, serde_json::Value)> {
    let trimmed = content.trim();

    // Case 1: the whole message is (optionally fenced) JSON - the common
    // template case where nothing but the call is printed.
    let unfenced = trimmed
        .strip_prefix("```json")
        .or_else(|| trimmed.strip_prefix("```"))
        .and_then(|s| s.strip_suffix("```"))
        .map(str::trim)
        .unwrap_or(trimmed);

    if unfenced.starts_with('{') {
        if let Some(call) = parse_tool_call_object(unfenced, offered) {
            return Some(call);
        }
    }

    // Case 2: the call is a ```json-fenced block embedded in prose. Some
    // models (qwen2.5-coder after a rejected call) explain themselves and
    // wrap the retry call in a fence: "Let's try again:\n```json\n{...}\n```".
    // Recover the FIRST fenced block that parses as a call to an offered
    // tool; the caller's loop re-prompts for any further calls. Only
    // *fenced* blocks are considered - a bare `{...}` mentioned inline in
    // prose is deliberately NOT treated as a call (that would fire on a
    // model merely describing JSON).
    for inner in fenced_json_blocks(content) {
        if let Some(call) = parse_tool_call_object(inner.trim(), offered) {
            return Some(call);
        }
    }

    None
}

/// Yield the inner text of each ```json ... ``` (or bare ``` ... ```) fenced
/// block in `content`, in order. Used to find tool calls a model wrapped in
/// a code fence inside otherwise-prose content.
fn fenced_json_blocks(content: &str) -> Vec<&str> {
    let mut blocks = Vec::new();
    let mut rest = content;
    while let Some(open) = rest.find("```") {
        // Skip past the opening fence and an optional language tag line.
        let after_open = &rest[open + 3..];
        let body_start = match after_open.find('\n') {
            Some(nl) => nl + 1,
            None => break, // opening fence with no newline -> no body
        };
        let body = &after_open[body_start..];
        let Some(close) = body.find("```") else {
            break; // unterminated fence
        };
        blocks.push(&body[..close]);
        rest = &body[close + 3..];
    }
    blocks
}

/// Parse a single JSON object as a strict tool call: it must name an
/// offered tool, carry an explicit `arguments`/`parameters` object, and
/// have no other keys. Shared by both the whole-message and embedded-fence
/// recovery paths.
fn parse_tool_call_object(text: &str, offered: &[Tool]) -> Option<(String, serde_json::Value)> {
    if !text.starts_with('{') {
        return None;
    }

    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let obj = value.as_object()?;

    let name = obj.get("name")?.as_str()?;
    if !offered.iter().any(|t| t.name == name) {
        tracing::debug!("Content looks like a tool call but names no offered tool: {name}");
        return None;
    }

    // The arguments object must be present and be an object. A bare
    // `{"name": "bash"}` is not a call, and defaulting it to `{}` would
    // invoke the tool with empty input on the strength of a guess.
    let arguments = obj.get("arguments").or_else(|| obj.get("parameters"))?;
    if !arguments.is_object() {
        return None;
    }
    let arguments = arguments.clone();

    // Any other key means this is not a bare tool call - don't guess.
    if obj
        .keys()
        .any(|k| !matches!(k.as_str(), "name" | "arguments" | "parameters"))
    {
        return None;
    }

    tracing::info!(
        "Recovered a tool call the model emitted as text (its native tool-call \
         mechanism, if any, did not carry it): {name}"
    );
    Some((name.to_string(), arguments))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bash_tool() -> Tool {
        Tool {
            name: "bash".to_string(),
            description: "Run a shell command".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {"command": {"type": "string"}},
                "required": ["command"],
            }),
        }
    }

    /// qwen2.5-coder's Ollama template never populates `tool_calls` - it prints
    /// the call as message content. Without recovering it, nothing executes and
    /// the user just sees raw JSON in the chat.
    #[test]
    fn tool_call_printed_as_content_is_recovered() {
        let tools = [bash_tool()];
        let content =
            "{\n  \"name\": \"bash\",\n  \"arguments\": {\n    \"command\": \"ls -la\"\n  }\n}";

        let (name, args) =
            tool_call_from_content(content, &tools).expect("call should be recovered");
        assert_eq!(name, "bash");
        assert_eq!(args["command"], "ls -la");
    }

    #[test]
    fn tool_call_in_a_json_fence_is_recovered() {
        let tools = [bash_tool()];
        let content = "```json\n{\"name\": \"bash\", \"arguments\": {\"command\": \"ls\"}}\n```";
        let (name, _) = tool_call_from_content(content, &tools).expect("recovered");
        assert_eq!(name, "bash");
    }

    /// Regression: qwen2.5-coder, after a rejected call, apologises in prose and
    /// wraps the retry call in a ```json fence. The call must be recovered from
    /// that fenced block rather than printed to the user as raw JSON.
    #[test]
    fn tool_call_in_a_fence_embedded_in_prose_is_recovered() {
        let tools = [bash_tool()];
        let content = "I apologize for that. Let's try again.\n\n\
             First, create the folder:\n\n\
             ```json\n{\"name\": \"bash\", \"arguments\": {\"command\": \"mkdir exercice1\"}}\n```\n\n\
             Then we can continue.";
        let (name, args) =
            tool_call_from_content(content, &tools).expect("fenced call in prose must recover");
        assert_eq!(name, "bash");
        assert_eq!(args["command"], "mkdir exercice1");
    }

    /// When several fenced calls appear, recover the FIRST offered-tool call;
    /// the agent loop re-prompts for the rest.
    #[test]
    fn first_of_several_fenced_calls_is_recovered() {
        let tools = [bash_tool()];
        let content = "Step 1:\n```json\n{\"name\": \"bash\", \"arguments\": {\"command\": \"mkdir a\"}}\n```\n\
             Step 2:\n```json\n{\"name\": \"bash\", \"arguments\": {\"command\": \"cargo new b\"}}\n```";
        let (name, args) = tool_call_from_content(content, &tools).expect("recovered");
        assert_eq!(name, "bash");
        assert_eq!(
            args["command"], "mkdir a",
            "the first fenced call must be the one recovered"
        );
    }

    /// A fenced block that is not a valid offered-tool call must not be
    /// recovered even though it is fenced (e.g. a code example the model shows).
    #[test]
    fn fenced_non_tool_json_is_not_recovered() {
        let tools = [bash_tool()];
        let content = "Here's the Cargo.toml structure:\n\
             ```json\n{\"package\": {\"name\": \"demo\"}}\n```";
        assert!(tool_call_from_content(content, &tools).is_none());
    }

    /// The recovery must never fire on content that merely *contains* JSON, or
    /// Crustly would execute a tool the model never asked for.
    #[test]
    fn prose_is_never_mistaken_for_a_tool_call() {
        let tools = [bash_tool()];
        for content in [
            "Here is an example: {\"name\": \"bash\", \"arguments\": {}}",
            "I will run ls for you.",
            "{\"name\": \"rm_rf\", \"arguments\": {}}", // not an offered tool
            "{\"name\": \"bash\"}",                     // no arguments object
            "{\"name\": \"bash\", \"arguments\": {}, \"note\": \"extra\"}", // unexpected key
            "{\"arguments\": {\"command\": \"ls\"}}",   // no name
            "{}",
            "",
        ] {
            assert!(
                tool_call_from_content(content, &tools).is_none(),
                "must not be treated as a tool call: {content:?}"
            );
        }
    }

    /// Prose must still stream token-by-token; only content that might be a
    /// printed tool call is withheld.
    #[test]
    fn only_json_like_content_is_withheld_from_streaming() {
        assert!(maybe_tool_call_json(""));
        assert!(maybe_tool_call_json("{"));
        assert!(maybe_tool_call_json("  {\"name\""));
        assert!(maybe_tool_call_json("```json"));
        assert!(!maybe_tool_call_json("Here are the files"));
        assert!(!maybe_tool_call_json("I'll run ls."));
    }

    /// `commits_to_an_offered_tool_call` gates grammar-constrained decoding
    /// (Phase 4b) - unlike `maybe_tool_call_json` above, a false positive
    /// here is actively harmful (it hijacks unrelated output into an
    /// executed tool call), so it must stay much stricter.
    #[test]
    fn commits_to_an_offered_tool_call_requires_a_real_tool_name_not_just_a_brace() {
        let tools = [bash_tool()];
        // A bare brace, or a name that isn't an offered tool, must not
        // qualify - only `maybe_tool_call_json` is allowed to be this loose.
        assert!(!commits_to_an_offered_tool_call("{", &tools));
        assert!(!commits_to_an_offered_tool_call("{\"foo\": 1}", &tools));
        assert!(!commits_to_an_offered_tool_call(
            "{\"name\": \"rm_rf\"",
            &tools
        ));
        // A JSON answer that merely starts with '{' and never names an
        // offered tool must never trigger, however long it runs.
        assert!(!commits_to_an_offered_tool_call(
            "{\"schema\": {\"type\": \"object\"}}",
            &tools
        ));
    }

    #[test]
    fn commits_to_an_offered_tool_call_recognizes_compact_and_spaced_name_keys() {
        let tools = [bash_tool()];
        assert!(commits_to_an_offered_tool_call(
            "{\"name\":\"bash\", \"arg",
            &tools
        ));
        assert!(commits_to_an_offered_tool_call(
            "{\"name\": \"bash\", \"arg",
            &tools
        ));
    }

    #[test]
    fn commits_to_an_offered_tool_call_rejects_a_fenced_block() {
        // Fenced blocks are left entirely to the post-hoc recovery
        // heuristic - a JSON-only grammar can't express the surrounding
        // prose a fence implies.
        let tools = [bash_tool()];
        assert!(!commits_to_an_offered_tool_call(
            "```json\n{\"name\": \"bash\"",
            &tools
        ));
    }

    #[test]
    fn commits_to_an_offered_tool_call_requires_leading_brace_not_just_a_substring_match() {
        // A name:tool substring appearing inside prose (not as the response's
        // own leading JSON) must not trigger - matches
        // `parse_tool_call_object`'s own "the whole content is the call"
        // strictness in spirit.
        let tools = [bash_tool()];
        assert!(!commits_to_an_offered_tool_call(
            "Sure, I'll use {\"name\": \"bash\"} as an example.",
            &tools
        ));
    }

    /// Regression: a nested `"name"` field (inside an array/sub-object) must
    /// not be mistaken for the response's own top-level tool-call name -
    /// otherwise a plain JSON answer that happens to embed the string
    /// `"name": "bash"` somewhere inside it would get hijacked into a
    /// fabricated tool call just as badly as the original "any leading `{`"
    /// bug this function was written to replace.
    #[test]
    fn commits_to_an_offered_tool_call_rejects_a_nested_name_field() {
        let tools = [bash_tool()];
        assert!(!commits_to_an_offered_tool_call(
            "{\"records\": [{\"id\": 1, \"name\": \"bash\"}], \"summary\": \"ok\"}",
            &tools
        ));
    }

    #[test]
    fn commits_to_an_offered_tool_call_accepts_the_top_level_name_after_other_keys() {
        // The offered-tool name key doesn't have to be literally the first
        // byte after '{' - just at depth 1, wherever it lands.
        let tools = [bash_tool()];
        assert!(commits_to_an_offered_tool_call(
            "{\"extra\": 1, \"name\": \"bash\", \"arg",
            &tools
        ));
    }

    #[test]
    fn leading_window_never_panics_and_stays_within_a_char_boundary() {
        // A multibyte character straddling the exact window cutoff must not
        // panic the slice - only ASCII is used in practice (tool names,
        // JSON syntax), but this must hold regardless.
        let text = "{\"name\": \"€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€\"}";
        let window = leading_window(text, NAME_KEY_SCAN_WINDOW);
        assert!(text.starts_with(window));
    }

    #[test]
    fn commits_to_an_offered_tool_call_does_not_scan_past_the_window() {
        // A tool name appearing only far past NAME_KEY_SCAN_WINDOW bytes in
        // must not trigger - the bound exists precisely so this check stays
        // O(1) regardless of response length, at the deliberate cost of
        // missing a "name" key an unusually verbose response pushed far
        // past where a real call would ever put it.
        let tools = [bash_tool()];
        let padding = "x".repeat(NAME_KEY_SCAN_WINDOW + 50);
        let text = format!("{{\"padding\": \"{padding}\", \"name\": \"bash\", \"arg");
        assert!(!commits_to_an_offered_tool_call(&text, &tools));
    }
}
