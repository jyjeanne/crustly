---
type: Rust Function
title: pull_model_forwards_progress_and_completes
resource: src/llm/provider/ollama_models.rs#L313-L338
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama_models/pull_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn pull_model_forwards_progress_and_completes()`

# Calls

- [pull_model](../../../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [is_success](../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)