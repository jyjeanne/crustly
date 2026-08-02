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
}
