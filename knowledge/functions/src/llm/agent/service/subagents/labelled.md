---
type: Rust Function
title: labelled
resource: src/llm/agent/service/subagents.rs#L105-L112
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn labelled(callback: &ApprovalCallback, description: &str) -> ApprovalCallback`

# Called by

- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)