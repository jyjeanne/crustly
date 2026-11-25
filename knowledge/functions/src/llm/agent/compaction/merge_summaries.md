---
type: Rust Function
title: merge_summaries
resource: src/llm/agent/compaction.rs#L251-L271
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/compaction/compact_keeping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/merge_summaries_drops_the_oldest_lines_past_the_cap
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn merge_summaries(previous: Option<String>, new: &str) -> String`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [compact_keeping](../../../../../functions/src/llm/agent/compaction/compact_keeping.md)
- [merge_summaries_drops_the_oldest_lines_past_the_cap](../../../../../functions/src/llm/agent/compaction/merge_summaries_drops_the_oldest_lines_past_the_cap.md)