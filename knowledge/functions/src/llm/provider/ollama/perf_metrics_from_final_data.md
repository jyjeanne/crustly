---
type: Rust Function
title: perf_metrics_from_final_data
resource: src/llm/provider/ollama.rs#L917-L926
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/on_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/test_perf_metrics_from_final_data
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn perf_metrics_from_final_data(final_data: &ChatMessageFinalResponseData) -> PerfMetrics`

# Called by

- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)
- [on_chunk](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/on_chunk.md)
- [test_perf_metrics_from_final_data](../../../../../functions/src/llm/provider/ollama/test_perf_metrics_from_final_data.md)