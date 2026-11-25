---
type: Rust Method
title: should_compact
resource: src/llm/agent/context.rs#L210-L213
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/compaction_fires_at_threshold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/compaction_test/should_compact_fires_at_80_percent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn should_compact(&self) -> bool`

# Called by

- [compaction_fires_at_threshold](../../../../../../functions/src/llm/agent/compaction/compaction_fires_at_threshold.md)
- [should_compact_fires_at_80_percent](../../../../../../functions/tests/compaction_test/should_compact_fires_at_80_percent.md)