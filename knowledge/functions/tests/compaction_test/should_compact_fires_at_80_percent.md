---
type: Rust Function
title: should_compact_fires_at_80_percent
resource: tests/compaction_test.rs#L143-L155
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/should_compact
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn should_compact_fires_at_80_percent()`

# Calls

- [new](../../../functions/src/llm/agent/context/AgentContext/new.md)
- [should_compact](../../../functions/src/llm/agent/context/AgentContext/should_compact.md)