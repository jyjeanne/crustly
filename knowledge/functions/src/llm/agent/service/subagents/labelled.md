---
type: Rust Function
title: labelled
resource: src/llm/agent/service/subagents.rs#L107-L114
visibility: private
generated:
  by: okf-rs/0.8.0
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