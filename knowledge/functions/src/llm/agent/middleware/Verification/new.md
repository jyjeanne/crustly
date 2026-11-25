---
type: Rust Method
title: new
resource: src/llm/agent/middleware.rs#L481-L488
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/verification_nudges_after_an_edit_until_a_check_passes
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/middleware/verification_gives_up_after_two_nudges
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/round_pipeline
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(command: impl Into<String>) -> Self`

# Called by

- [verification_nudges_after_an_edit_until_a_check_passes](../../../../../../functions/src/llm/agent/middleware/verification_nudges_after_an_edit_until_a_check_passes.md)
- [verification_gives_up_after_two_nudges](../../../../../../functions/src/llm/agent/middleware/verification_gives_up_after_two_nudges.md)
- [round_pipeline](../../../../../../functions/src/llm/agent/service/AgentService/round_pipeline.md)