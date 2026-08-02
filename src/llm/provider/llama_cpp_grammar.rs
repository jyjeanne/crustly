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
//! ## How this is wired into `run_complete`/`run_stream`
//!
//! A grammar built from `tool_call_json_schema` matches *only* a tool call -
//! there is no JSON Schema expression for "or arbitrary free-form text".
//! Chaining this sampler unconditionally for the whole generation whenever
//! `request.tools` is non-empty (which, in Crustly's agent loop, is nearly
//! every turn) would make it *impossible* for the model to ever answer in
//! plain text again once tools are offered - a severe regression, not an
//! additive reliability upgrade. So it isn't chained unconditionally.
//!
//! Instead, `run_complete`/`run_stream` (`llama_cpp.rs`) decode unconstrained
//! as normal, checking the *whole* response generated so far (never a
//! withheld/reset suffix - see the `pending_flush`-vs-`full_text` note
//! below) against `tool_call_recovery::commits_to_an_offered_tool_call` on
//! every token. Unlike the looser `maybe_tool_call_json` `run_stream` uses
//! to decide what to *withhold* from the live stream, this trigger only
//! fires once the model has typed a bare, unfenced `{"name": "<tool>"` for
//! a *real* offered tool - not merely opened a brace. That distinction
//! matters: a trigger as loose as "starts with `{`" would engage the
//! grammar on any JSON-shaped free-text answer (asked for by a user, not a
//! tool call at all) whenever tools happen to be offered - which is nearly
//! every turn in Crustly's agent loop - permanently forcing that unrelated
//! answer into a fabricated, then-executed tool call. Once the stricter
//! trigger fires, `try_build_constrained_sampler()` builds this module's
//! sampler, replays the tokens generated so far into the *entire* new chain
//! via `accept_many()` (not just the grammar stage - see that function's
//! doc comment for why the whole chain needs it), and the result becomes
//! the sampler for the rest of that response. A plain-text response, or a
//! JSON-shaped answer that never names a real tool, never reaches the
//! trigger and is never constrained; a genuine tool-call response gets
//! syntax-guaranteed JSON instead of relying solely on the always-on
//! recovery heuristic (`tool_call_recovery.rs`) after the fact - which
//! still runs regardless, as the final safety net.
//!
//! `run_stream` in particular must check `full_text` (accumulated for the
//! whole response, never reset), not `pending_flush` (emptied by
//! `std::mem::take` every time withheld content turns out to be ordinary
//! prose and gets flushed) - checking the latter let the trigger and the
//! `generated_tokens` replay disagree about which window of the response
//! they were each looking at, letting already-flushed prose tokens get
//! replayed into a JSON-only grammar (poisoning it) or letting a swap fire
//! on a *resumed* block of text rather than the response's true start.
//!
//! **Not verified against a real `.gguf` model** - none is available in this
//! sandbox (network access to fetch one is also blocked here). What *is*
//! confirmed, by reading `llguidance` 1.7.6's `Matcher::consume_tokens`/
//! `with_inner` and `llama-cpp-2` 0.1.153's `llg_accept`/`llg_apply` directly:
//! a token-replay mismatch during `accept_many()` poisons the `Matcher` into
//! an `Error` state, after which `llg_apply` becomes a documented no-op and
//! `llg_accept` silently ignores further errors - the constrained sampler
//! just stops masking anything from that point on, silently reverting to the
//! exact unconstrained behavior this feature is additive on top of, with
//! `tool_call_recovery.rs` still catching whatever comes out. That's the
//! actual worst case: no improvement for that one response, not a hang,
//! panic, or corrupted decode loop - see
//! `try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay`
//! (`llama_cpp.rs`) for the offline test exercising exactly this.

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

/// Build the `llguidance` [`ParserFactory`](llguidance::ParserFactory) for a
/// model's tokenizer environment.
///
/// `tok_env` is itself expensive to build (walks the entire vocabulary -
/// see [`LlamaSampler::llguidance_tok_env`]'s own doc comment), and the
/// factory built from it is - per `llguidance`'s own doc comment on
/// [`ParserFactory`](llguidance::ParserFactory) - "typically created once
/// per model/tokenizer and reused across requests". Callers build this once
/// per model load (alongside `tok_env` itself) and hold onto it, never
/// rebuilding it per request or per grammar - see `llama_cpp.rs`'s
/// `ToolCallGrammarEnv`.
pub fn build_parser_factory(
    tok_env: &toktrie::TokEnv,
) -> std::result::Result<llguidance::ParserFactory, String> {
    llguidance::ParserFactory::new_simple(tok_env).map_err(|e| e.to_string())
}

/// Build an [`LlamaSampler`] that constrains decoding to only tokens
/// consistent with [`tool_call_json_schema`] for `tools`, using an
/// already-built `factory` (see [`build_parser_factory`] - cheap to call
/// repeatedly, unlike building the factory itself).
pub fn build_tool_call_sampler(
    factory: &llguidance::ParserFactory,
    tools: &[Tool],
) -> std::result::Result<LlamaSampler, String> {
    let schema = tool_call_json_schema(tools);
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
        let factory = build_parser_factory(&tok_env).expect("factory must build");
        let sampler = build_tool_call_sampler(&factory, &[bash_tool(), read_tool()]);
        assert!(
            sampler.is_ok(),
            "expected the tool-call schema to compile into a valid llguidance parser: {:?}",
            sampler.err()
        );
    }

    /// A `ParserFactory` is meant to be built once and reused (see its own
    /// doc comment) - confirm it actually can be, across two different
    /// grammars, since `build_parser_factory` is only ever called once per
    /// model load in the real wiring.
    #[test]
    fn one_factory_builds_multiple_independent_grammars() {
        let tok_env = toktrie::ApproximateTokEnv::single_byte_env();
        let factory = build_parser_factory(&tok_env).expect("factory must build");

        let first = build_tool_call_sampler(&factory, &[bash_tool()]);
        let second = build_tool_call_sampler(&factory, &[bash_tool(), read_tool()]);
        assert!(first.is_ok());
        assert!(second.is_ok());
    }
}
