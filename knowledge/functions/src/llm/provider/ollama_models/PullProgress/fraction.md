---
type: Rust Method
title: fraction
resource: src/llm/provider/ollama_models.rs#L47-L51
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama_models/pull_progress_fraction
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama_models/pull_progress_fraction_missing_data
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn fraction(&self) -> Option<f64>`

# Called by

- [pull_progress_fraction](../../../../../../functions/src/llm/provider/ollama_models/pull_progress_fraction.md)
- [pull_progress_fraction_missing_data](../../../../../../functions/src/llm/provider/ollama_models/pull_progress_fraction_missing_data.md)