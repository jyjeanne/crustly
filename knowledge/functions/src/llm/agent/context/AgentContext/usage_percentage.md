---
type: Rust Method
title: usage_percentage
resource: src/llm/agent/context.rs#L201-L203
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/compaction_fires_at_threshold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/test_usage_percentage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn usage_percentage(&self) -> f64`

# Called by

- [compaction_fires_at_threshold](../../../../../../functions/src/llm/agent/compaction/compaction_fires_at_threshold.md)
- [test_usage_percentage](../../../../../../functions/src/llm/agent/context/test_usage_percentage.md)