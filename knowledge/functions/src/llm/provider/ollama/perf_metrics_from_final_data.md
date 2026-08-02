---
type: Rust Function
title: perf_metrics_from_final_data
resource: src/llm/provider/ollama.rs#L859-L868
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
  - functions/src/llm/provider/ollama/OllamaProvider/provider/stream
  - functions/src/llm/provider/ollama/test_perf_metrics_from_final_data
---

# Signature

`fn perf_metrics_from_final_data(final_data: &ChatMessageFinalResponseData) -> PerfMetrics`

# Called by

- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)
- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [test_perf_metrics_from_final_data](../../../../../functions/src/llm/provider/ollama/test_perf_metrics_from_final_data.md)