---
type: Rust Method
title: tokens_per_second
resource: src/llm/provider/types.rs#L335-L338
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/test_perf_metrics_from_final_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/perf_metrics_tokens_per_second
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/perf_metrics_tokens_per_second_missing_duration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/perf_metrics_tokens_per_second_zero_duration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/complete_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn tokens_per_second(&self, output_tokens: u32) -> Option<f64>`

# Called by

- [test_perf_metrics_from_final_data](../../../../../../functions/src/llm/provider/ollama/test_perf_metrics_from_final_data.md)
- [perf_metrics_tokens_per_second](../../../../../../functions/src/llm/provider/types/perf_metrics_tokens_per_second.md)
- [perf_metrics_tokens_per_second_missing_duration](../../../../../../functions/src/llm/provider/types/perf_metrics_tokens_per_second_missing_duration.md)
- [perf_metrics_tokens_per_second_zero_duration](../../../../../../functions/src/llm/provider/types/perf_metrics_tokens_per_second_zero_duration.md)
- [complete_response](../../../../../../functions/src/tui/app/App/complete_response.md)