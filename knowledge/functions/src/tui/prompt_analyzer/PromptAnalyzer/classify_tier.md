---
type: Rust Method
title: classify_tier
resource: src/tui/prompt_analyzer.rs#L252-L278
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/tests/model_routing_test/complex_prompt_routes_to_powerful_tier
  - functions/tests/model_routing_test/simple_prompt_routes_to_fast_tier
  - functions/tests/model_routing_test/neutral_prompt_routes_to_balanced_tier
---

# Signature

`pub fn classify_tier(&self, prompt: &str) -> crate::llm::provider::router::ModelTier`

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [complex_prompt_routes_to_powerful_tier](../../../../../functions/tests/model_routing_test/complex_prompt_routes_to_powerful_tier.md)
- [simple_prompt_routes_to_fast_tier](../../../../../functions/tests/model_routing_test/simple_prompt_routes_to_fast_tier.md)
- [neutral_prompt_routes_to_balanced_tier](../../../../../functions/tests/model_routing_test/neutral_prompt_routes_to_balanced_tier.md)