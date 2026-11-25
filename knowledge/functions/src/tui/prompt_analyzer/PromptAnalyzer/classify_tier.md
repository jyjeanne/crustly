---
type: Rust Method
title: classify_tier
resource: src/tui/prompt_analyzer.rs#L265-L291
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/model_routing_test/complex_prompt_routes_to_powerful_tier
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/model_routing_test/simple_prompt_routes_to_fast_tier
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/model_routing_test/neutral_prompt_routes_to_balanced_tier
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn classify_tier(&self, prompt: &str) -> crate::llm::provider::router::ModelTier`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [complex_prompt_routes_to_powerful_tier](../../../../../functions/tests/model_routing_test/complex_prompt_routes_to_powerful_tier.md)
- [simple_prompt_routes_to_fast_tier](../../../../../functions/tests/model_routing_test/simple_prompt_routes_to_fast_tier.md)
- [neutral_prompt_routes_to_balanced_tier](../../../../../functions/tests/model_routing_test/neutral_prompt_routes_to_balanced_tier.md)