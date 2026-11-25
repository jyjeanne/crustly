---
type: Rust Function
title: held_response_server
resource: src/llm/provider/test_support.rs#L13-L45
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/test_support/read_request
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) async fn held_response_server( content_type: &'static str, first: String, rest: String, release: oneshot::Receiver<()>, ) -> String`

# Calls

- [read_request](../../../../../functions/src/llm/provider/test_support/read_request.md)

# Called by

- [text_arrives_before_the_response_is_complete](../../../../../functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete.md)
- [a_reply_cut_off_before_the_done_chunk_is_an_error](../../../../../functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error.md)
- [text_arrives_before_the_response_is_complete](../../../../../functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete.md)