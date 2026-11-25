---
type: Rust Method
title: drifted
resource: src/memory/anchors.rs#L254-L285
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/anchors/MemoryStore/ready_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/bundle_time
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/anchors_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/closest
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn drifted(&self, project: &ProjectIdentity) -> Result<Vec<Drift>>`

# Calls

- [ready_code](../../../../../functions/src/memory/anchors/MemoryStore/ready_code.md)
- [bundle_time](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_time.md)
- [anchors_for](../../../../../functions/src/db/repository/memory/MemoryRepository/anchors_for.md)
- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [get](../../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [closest](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/closest.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one](../../../../../functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one.md)