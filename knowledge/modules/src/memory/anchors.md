---
type: Rust Module
title: anchors
resource: src/memory/anchors.rs#L1-L286
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-code-knowledge-codeknowledge-detection-max-anchors-max-neighbour-sources
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-model-memory-memoryscope-memorystatus-projectid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-project-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-memorystore
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Drift](../../../classes/src/memory/anchors/Drift.md)
- [AnchorHits](../../../classes/src/memory/anchors/AnchorHits.md)
- [with_code_knowledge](../../../functions/src/memory/anchors/MemoryStore/with_code_knowledge.md)
- [code_knowledge](../../../functions/src/memory/anchors/MemoryStore/code_knowledge.md)
- [code_knowledge_status](../../../functions/src/memory/anchors/MemoryStore/code_knowledge_status.md)
- [start_code_knowledge_in_background](../../../functions/src/memory/anchors/MemoryStore/start_code_knowledge_in_background.md)
- [start_code_knowledge](../../../functions/src/memory/anchors/MemoryStore/start_code_knowledge.md)
- [ready_code](../../../functions/src/memory/anchors/MemoryStore/ready_code.md)
- [anchor_automatically](../../../functions/src/memory/anchors/MemoryStore/anchor_automatically.md)
- [set_anchors](../../../functions/src/memory/anchors/MemoryStore/set_anchors.md)
- [anchors](../../../functions/src/memory/anchors/MemoryStore/anchors.md)
- [anchored_to](../../../functions/src/memory/anchors/MemoryStore/anchored_to.md)
- [anchor_hits](../../../functions/src/memory/anchors/MemoryStore/anchor_hits.md)
- [anchored_in_file](../../../functions/src/memory/anchors/MemoryStore/anchored_in_file.md)
- [anchor_locations](../../../functions/src/memory/anchors/MemoryStore/anchor_locations.md)
- [drifted](../../../functions/src/memory/anchors/MemoryStore/drifted.md)

# Imports

- `super::code_knowledge::{CodeKnowledge, Detection, MAX_ANCHORS, MAX_NEIGHBOUR_SOURCES}`
- `super::model::{Memory, MemoryScope, MemoryStatus, ProjectId}`
- `super::project::ProjectIdentity`
- `super::MemoryStore`
- `anyhow::Result`
- `chrono::{DateTime, Utc}`
- `std::collections::{HashMap, HashSet}`
- `std::sync::Arc`
- `uuid::Uuid`

# Member of

- [crustly](../../../packages/crustly.md)