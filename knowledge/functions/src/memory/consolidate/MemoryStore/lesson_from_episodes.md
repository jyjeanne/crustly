---
type: Rust Method
title: lesson_from_episodes
resource: src/memory/consolidate.rs#L180-L256
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/folded_sources
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_temperature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/parse_lesson
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/add_sources
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn lesson_from_episodes( &self, project: &ProjectIdentity, active: &[Memory], gone: &HashSet<Uuid>, ) -> Result<usize>`

# Calls

- [folded_sources](../../../../../functions/src/db/repository/memory/MemoryRepository/folded_sources.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_temperature](../../../../../functions/src/llm/provider/types/LLMRequest/with_temperature.md)
- [parse_lesson](../../../../../functions/src/memory/consolidate/parse_lesson.md)
- [save](../../../../../functions/src/memory/MemoryStore/save.md)
- [add_sources](../../../../../functions/src/db/repository/memory/MemoryRepository/add_sources.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)

# Called by

- [consolidate](../../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)