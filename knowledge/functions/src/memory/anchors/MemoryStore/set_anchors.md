---
type: Rust Method
title: set_anchors
resource: src/memory/anchors.rs#L117-L147
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/anchors/MemoryStore/ready_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/closest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/bundle_time
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/set_anchors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_code_knowledge_test/naming_the_concept_or_a_caller_recalls_the_anchored_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn set_anchors( &self, id: Uuid, concepts: &[String], ) -> Result<std::result::Result<(), String>>`

# Calls

- [get](../../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [ready_code](../../../../../functions/src/memory/anchors/MemoryStore/ready_code.md)
- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [closest](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/closest.md)
- [bundle_time](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_time.md)
- [set_anchors](../../../../../functions/src/db/repository/memory/MemoryRepository/set_anchors.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [naming_the_concept_or_a_caller_recalls_the_anchored_memory](../../../../../functions/tests/memory_code_knowledge_test/naming_the_concept_or_a_caller_recalls_the_anchored_memory.md)