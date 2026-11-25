---
type: Rust Method
title: with_verification
resource: src/llm/agent/service.rs#L636-L640
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_verification(mut self, enabled: bool, command: Option<String>) -> Self`

# Called by

- [configure_agent_loop](../../../../../../functions/src/cli/configure_agent_loop.md)
- [verification_sends_the_model_back_to_check_its_edit](../../../../../../functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)