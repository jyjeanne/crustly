---
type: Rust Method
title: call_provider_streaming
resource: src/llm/agent/service.rs#L1518-L1530
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/types/LLMRequest/with_streaming
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/drain_stream_to_response
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn call_provider_streaming( provider: &Arc<dyn Provider>, request: LLMRequest, chunk_tx: Option<&mpsc::UnboundedSender<String>>, model_name: &str, ) -> crate::llm::provider::Result<LLMResponse>`

# Calls

- [with_streaming](../../../../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [drain_stream_to_response](../../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)

# Called by

- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)