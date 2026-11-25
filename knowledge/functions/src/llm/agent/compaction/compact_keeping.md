---
type: Rust Function
title: compact_keeping
resource: src/llm/agent/compaction.rs#L64-L135
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/compaction/plan_compaction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/summarise_turns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/merge_summaries
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/compaction/compact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn compact_keeping( ctx: &mut AgentContext, pool: &sqlx::SqlitePool, keep: usize, ) -> Result<CompactionRecord>`

# Calls

- [plan_compaction](../../../../../functions/src/llm/agent/compaction/plan_compaction.md)
- [summarise_turns](../../../../../functions/src/llm/agent/compaction/summarise_turns.md)
- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [merge_summaries](../../../../../functions/src/llm/agent/compaction/merge_summaries.md)

# Called by

- [compact](../../../../../functions/src/llm/agent/compaction/compact.md)
- [compaction_of_one_long_request_keeps_the_task_and_latest_rounds](../../../../../functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds.md)
- [compaction_refuses_when_nothing_would_be_dropped](../../../../../functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped.md)
- [make_room](../../../../../functions/src/llm/agent/service/AgentService/make_room.md)