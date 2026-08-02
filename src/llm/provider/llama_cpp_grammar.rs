//! Grammar-constrained tool-call decoding for `LlamaCppProvider`
//! (`llama-cpp-2-integration-plan.md` Phase 4b, optional, behind the
//! `llama-cpp-llguidance` Cargo feature).
//!
//! This module builds the two pieces Phase 4b needs: a JSON Schema that
//! describes "a valid call to one of the offered tools"
//! ([`tool_call_json_schema`]), and an [`LlamaSampler`] that constrains
//! decoding to only tokens consistent with that schema
//! ([`build_tool_call_sampler`]), via the `llguidance` crate's
//! `ParserFactory` -> `TopLevelGrammar` -> `Matcher` -> `LlamaSampler`
//! pipeline that `llama-cpp-2` 0.1.153 exposes (`LlamaSampler::llguidance_tok_env`,
//! `impl From<Matcher> for LlamaSampler`).
//!
//! ## Why this isn't wired into `run_complete`/`run_stream` yet
//!
//! A grammar built from `tool_call_json_schema` matches *only* a tool call -
//! there is no JSON Schema expression for "or arbitrary free-form text".
//! Chaining this sampler unconditionally for the whole generation whenever
//! `request.tools` is non-empty (which, in Crustly's agent loop, is nearly
//! every turn) would make it *impossible* for the model to ever answer in
//! plain text again once tools are offered - a severe regression, not an
//! additive reliability upgrade.
//!
//! The correct integration is a **mid-stream sampler swap**: decode
//! unconstrained as today, reuse the same
//! `tool_call_recovery::maybe_tool_call_json` trigger `run_stream` already
//! uses to withhold output, and only once that trigger fires (the model has
//! committed to something that looks like a call, typically after the very
//! first non-whitespace token) build this constrained sampler, replay the
//! handful of already-generated tokens into its `Matcher` via
//! `consume_tokens`, and decode the remainder of that response constrained.
//! A plain-text response never reaches the trigger and is never constrained,
//! preserving free-text answers exactly as Phase 4 already does; a tool-call
//! response gets syntax-guaranteed JSON instead of relying solely on the
//! always-on recovery heuristic (`tool_call_recovery.rs`) after the fact.
//!
//! That swap touches `run_complete`/`run_stream`'s live decode loop and its
//! correctness (in particular, whether `Matcher::consume_tokens` replaying a
//! few already-decoded tokens keeps the parser state genuinely in sync with
//! `llama.cpp`'s own KV cache) is not verifiable in this environment - no
//! `.gguf` model is available to run it against, and `llama-cpp-2-integration-plan.md`
//! §13 Phase 4b's own exit criteria requires exactly that manual, real-model
//! verification. Shipping the swap unverified would risk shipping a decode
//! loop that hangs, panics, or silently corrupts output for every user who
//! enables `llama-cpp-llguidance` - worse than not having the feature.
//!
//! So this pass ships the infrastructure - correct, offline-testable,
//! feature-gated - without wiring the swap into the hot path. The next step,
//! once real-model testing is possible, is exactly the swap described above.

use super::types::Tool;
use llama_cpp_2::sampling::LlamaSampler;

/// Build a JSON Schema matching exactly "a call to one of `tools`": an
/// object with a `name` naming one specific offered tool and an `arguments`
/// object matching that tool's own `input_schema`.
///
/// Pure and offline-testable - no model or `llguidance` parser involved.
/// The shape matches what `tool_call_recovery::parse_tool_call_object`
/// already accepts (and what `tool_instructions_block` already instructs
/// the model to produce): `{"name": "<tool_name>", "arguments": {...}}`,
/// no other keys. Only `arguments` is offered here, not the `parameters`
/// alias `parse_tool_call_object` also tolerates, since the instructed
/// format only ever tells the model to use `arguments` - the grammar should
/// constrain to what we asked for, not everything recovery happens to
/// forgive.
pub fn tool_call_json_schema(tools: &[Tool]) -> serde_json::Value {
    let variants: Vec<serde_json::Value> = tools
        .iter()
        .map(|tool| {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "name": { "const": tool.name },
                    "arguments": tool.input_schema,
                },
                "required": ["name", "arguments"],
                "additionalProperties": false,
            })
        })
        .collect();

    serde_json::json!({ "oneOf": variants })
}

/// Build an [`LlamaSampler`] that constrains decoding to only tokens
/// consistent with [`tool_call_json_schema`] for `tools`.
///
/// `tok_env` is expensive to build (walks the entire vocabulary - see
/// [`LlamaSampler::llguidance_tok_env`]'s own doc comment) and should be
/// built once per loaded model and reused, not rebuilt per call. Not wired
/// into any live decode path yet - see this module's doc comment.
pub fn build_tool_call_sampler(
    tok_env: &toktrie::TokEnv,
    tools: &[Tool],
) -> std::result::Result<LlamaSampler, String> {
    let schema = tool_call_json_schema(tools);
    let factory = llguidance::ParserFactory::new_simple(tok_env).map_err(|e| e.to_string())?;
    let grammar = llguidance::api::TopLevelGrammar::from_json_schema(schema);
    let parser = factory.create_parser(grammar);
    let matcher = llguidance::Matcher::new(parser);
    if matcher.is_error() {
        return Err(matcher
            .get_error()
            .unwrap_or_else(|| "llguidance matcher entered an error state".to_string()));
    }
    Ok(LlamaSampler::from(matcher))
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

    fn read_tool() -> Tool {
        Tool {
            name: "read".to_string(),
            description: "Read a file".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {"path": {"type": "string"}},
                "required": ["path"],
            }),
        }
    }

    #[test]
    fn schema_has_one_variant_per_offered_tool() {
        let schema = tool_call_json_schema(&[bash_tool(), read_tool()]);
        let variants = schema["oneOf"].as_array().expect("oneOf array");
        assert_eq!(variants.len(), 2);
    }

    #[test]
    fn each_variant_pins_the_tool_name_and_embeds_its_input_schema() {
        let schema = tool_call_json_schema(&[bash_tool()]);
        let variant = &schema["oneOf"][0];
        assert_eq!(variant["properties"]["name"]["const"], "bash");
        assert_eq!(
            variant["properties"]["arguments"]["properties"]["command"]["type"],
            "string"
        );
        assert_eq!(
            variant["required"],
            serde_json::json!(["name", "arguments"])
        );
        assert_eq!(variant["additionalProperties"], false);
    }

    #[test]
    fn empty_tool_list_produces_an_empty_oneof() {
        let schema = tool_call_json_schema(&[]);
        assert_eq!(schema["oneOf"].as_array().expect("oneOf array").len(), 0);
    }

    /// The schema itself must be valid JSON Schema `llguidance` can compile -
    /// exercised end-to-end (schema -> grammar -> parser -> matcher -> sampler)
    /// without needing a real `LlamaModel`, using `toktrie::ApproximateTokEnv`'s
    /// own single-byte test tokenizer environment as a stand-in `TokEnv`.
    #[test]
    fn schema_compiles_into_a_working_sampler() {
        let tok_env = toktrie::ApproximateTokEnv::single_byte_env();
        let sampler = build_tool_call_sampler(&tok_env, &[bash_tool(), read_tool()]);
        assert!(
            sampler.is_ok(),
            "expected the tool-call schema to compile into a valid llguidance parser: {:?}",
            sampler.err()
        );
    }
}
