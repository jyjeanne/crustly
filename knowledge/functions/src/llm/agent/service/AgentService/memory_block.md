---
type: Rust Method
title: memory_block
resource: src/llm/agent/service.rs#L645-L684
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/trigger
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn memory_block( &self, context: &AgentContext, message: &str, window: u32, ) -> Option<String>`

# Calls

- [identify](../../../../../../functions/src/memory/project/identify.md)
- [register_project](../../../../../../functions/src/memory/MemoryStore/register_project.md)
- [trigger](../../../../../../functions/src/memory/extraction/MemoryStore/trigger.md)
- [find](../../../../../../functions/src/memory/MemoryStore/find.md)
- [recall](../../../../../../functions/src/memory/MemoryStore/recall.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)