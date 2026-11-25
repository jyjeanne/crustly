---
type: Rust Module
title: extraction
resource: src/memory/extraction.rs#L1-L330
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-extract-self-turn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-model-memoryscope-memorysource-memorystatus-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-project-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-memorystore
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-contentblock-llmrequest-message-provider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-duration-utc
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

- [ExtractionHandle](../../../classes/src/memory/extraction/ExtractionHandle.md)
- [with_extraction](../../../functions/src/memory/extraction/MemoryStore/with_extraction.md)
- [trigger](../../../functions/src/memory/extraction/MemoryStore/trigger.md)
- [extraction_on](../../../functions/src/memory/extraction/MemoryStore/extraction_on.md)
- [has_unread_turns](../../../functions/src/memory/extraction/MemoryStore/has_unread_turns.md)
- [finish_session](../../../functions/src/memory/extraction/MemoryStore/finish_session.md)
- [extract](../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [record_failure](../../../functions/src/memory/extraction/MemoryStore/record_failure.md)
- [approve](../../../functions/src/memory/extraction/MemoryStore/approve.md)
- [pending](../../../functions/src/memory/extraction/MemoryStore/pending.md)

# Imports

- `super::extract::{self, Turn}`
- `super::model::{MemoryScope, MemorySource, MemoryStatus, NewMemory, SaveOutcome}`
- `super::project::ProjectIdentity`
- `super::MemoryStore`
- `crate::llm::provider::{ContentBlock, LLMRequest, Message, Provider}`
- `anyhow::{Context, Result}`
- `chrono::{Duration, Utc}`
- `std::sync::Arc`
- `uuid::Uuid`

# Member of

- [crustly](../../../packages/crustly.md)