---
type: Rust Function
title: drain_stream_to_response
resource: src/llm/agent/service.rs#L413-L539
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/apply_streamed_tool_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/route_text_delta
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/extract_think_tags
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/call_provider_streaming
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/drain_stream_to_response_carries_perf_metrics_through
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/drain_stream_assembles_anthropic_tool_input_from_json_deltas
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn drain_stream_to_response( stream: ProviderStream, chunk_tx: Option<&mpsc::UnboundedSender<String>>, model_name: &str, ) -> crate::llm::provider::Result<LLMResponse>`

# Calls

- [apply_streamed_tool_input](../../../../../functions/src/llm/agent/service/apply_streamed_tool_input.md)
- [route_text_delta](../../../../../functions/src/llm/agent/service/route_text_delta.md)
- [extract_think_tags](../../../../../functions/src/llm/provider/types/extract_think_tags.md)

# Called by

- [call_provider_streaming](../../../../../functions/src/llm/agent/service/AgentService/call_provider_streaming.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [drain_stream_to_response_carries_perf_metrics_through](../../../../../functions/src/llm/agent/service/drain_stream_to_response_carries_perf_metrics_through.md)
- [drain_stream_assembles_anthropic_tool_input_from_json_deltas](../../../../../functions/src/llm/agent/service/drain_stream_assembles_anthropic_tool_input_from_json_deltas.md)