---
type: Rust Function
title: last_user_text
resource: src/llm/agent/service.rs#L3089-L3102
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/round_limit_ends_with_an_answer_instead_of_an_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn last_user_text(request: &LLMRequest) -> String`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [verification_sends_the_model_back_to_check_its_edit](../../../../../functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit.md)
- [round_limit_ends_with_an_answer_instead_of_an_error](../../../../../functions/src/llm/agent/service/round_limit_ends_with_an_answer_instead_of_an_error.md)
- [stopping_mid_round_answers_every_pending_tool_call](../../../../../functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call.md)