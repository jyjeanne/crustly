---
type: Rust Method
title: stream_url
resource: src/llm/provider/gemini.rs#L105-L110
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn stream_url(&self, model: &str) -> String`

# Called by

- [stream](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)