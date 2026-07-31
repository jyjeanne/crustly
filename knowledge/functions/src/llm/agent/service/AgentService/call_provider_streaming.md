---
type: Rust Method
title: call_provider_streaming
resource: src/llm/agent/service.rs#L1579-L1591
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/types/LLMRequest/with_streaming
  - functions/src/llm/agent/service/drain_stream_to_response
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
---

# Signature

`async fn call_provider_streaming( provider: &Arc<dyn Provider>, request: LLMRequest, chunk_tx: Option<&mpsc::UnboundedSender<String>>, model_name: &str, ) -> crate::llm::provider::Result<LLMResponse>`

# Calls

- [with_streaming](../../../../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [drain_stream_to_response](../../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)