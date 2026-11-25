---
type: Rust Method
title: estimate_message_tokens
resource: src/llm/agent/context.rs#L166-L193
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/estimate_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/context/AgentContext/recount_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/trim_to_fit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn estimate_message_tokens(&self, message: &Message) -> usize`

# Calls

- [estimate_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/estimate_tokens.md)

# Called by

- [recount_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/recount_tokens.md)
- [add_message](../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [trim_to_fit](../../../../../../functions/src/llm/agent/context/AgentContext/trim_to_fit.md)