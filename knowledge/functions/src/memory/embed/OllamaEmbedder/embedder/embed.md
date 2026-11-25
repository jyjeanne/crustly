---
type: Rust Method
title: embed
resource: src/memory/embed.rs#L198-L231
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/embed/Prefixes/apply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiPart/text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/finish
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/ollama_reports_an_error_status_without_panicking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/ollama_refuses_a_wrong_number_of_vectors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/an_unreachable_server_is_a_transport_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn embed( &self, purpose: EmbedPurpose, texts: &[String], ) -> Result<Vec<Vec<f32>>, EmbedError>`

# Calls

- [apply](../../../../../../functions/src/memory/embed/Prefixes/apply.md)
- [status](../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [is_success](../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [text](../../../../../../functions/src/llm/provider/gemini/GeminiPart/text.md)
- [finish](../../../../../../functions/src/memory/embed/finish.md)

# Called by

- [ollama_sends_prefixed_input_and_returns_unit_vectors](../../../../../../functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors.md)
- [ollama_reports_an_error_status_without_panicking](../../../../../../functions/src/memory/embed/ollama_reports_an_error_status_without_panicking.md)
- [ollama_refuses_a_wrong_number_of_vectors](../../../../../../functions/src/memory/embed/ollama_refuses_a_wrong_number_of_vectors.md)
- [an_unreachable_server_is_a_transport_error](../../../../../../functions/src/memory/embed/an_unreachable_server_is_a_transport_error.md)