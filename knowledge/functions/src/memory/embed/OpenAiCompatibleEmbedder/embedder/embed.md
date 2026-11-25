---
type: Rust Method
title: embed
resource: src/memory/embed.rs#L273-L316
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
  - target: functions/src/memory/embed/openai_orders_by_index_and_sends_the_key
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

- [openai_orders_by_index_and_sends_the_key](../../../../../../functions/src/memory/embed/openai_orders_by_index_and_sends_the_key.md)