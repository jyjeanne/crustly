---
type: Rust Method
title: invalidate_matching
resource: src/llm/tools/cache.rs#L132-L134
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others
---

# Signature

`pub fn invalidate_matching(&self, pred: impl Fn(&str) -> bool)`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [invalidate_matching_drops_selected_tools_and_keeps_others](../../../../../../functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others.md)