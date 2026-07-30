---
type: Rust Method
title: handle_error
resource: src/llm/provider/gemini.rs#L339-L348
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/gemini/build_gemini_error
---

# Signature

`async fn handle_error(&self, response: reqwest::Response) -> ProviderError`

# Calls

- [build_gemini_error](../../../../../../functions/src/llm/provider/gemini/build_gemini_error.md)