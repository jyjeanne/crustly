---
type: Rust Function
title: pool_with_compaction_table
resource: src/llm/agent/compaction.rs#L414-L432
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn pool_with_compaction_table() -> sqlx::SqlitePool`

# Called by

- [compaction_of_one_long_request_keeps_the_task_and_latest_rounds](../../../../../functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds.md)
- [compaction_refuses_when_nothing_would_be_dropped](../../../../../functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped.md)