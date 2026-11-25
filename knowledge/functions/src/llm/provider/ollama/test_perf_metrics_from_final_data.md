---
type: Rust Function
title: test_perf_metrics_from_final_data
resource: src/llm/provider/ollama.rs#L1144-L1160
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/perf_metrics_from_final_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/PerfMetrics/tokens_per_second
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_perf_metrics_from_final_data()`

# Calls

- [perf_metrics_from_final_data](../../../../../functions/src/llm/provider/ollama/perf_metrics_from_final_data.md)
- [tokens_per_second](../../../../../functions/src/llm/provider/types/PerfMetrics/tokens_per_second.md)