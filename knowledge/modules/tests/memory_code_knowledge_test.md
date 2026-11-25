---
type: Rust Module
title: memory_code_knowledge_test
resource: tests/memory_code_knowledge_test.rs#L1-L498
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-config-memoryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-mcp-client-mcptooldef
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-code-knowledge-codeknowledge-connector-detection-okfserver
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorycategory-memoryscope-memorysource-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-project-id-for-root-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-memorystore-recallrequest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
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

- [Concept](../../classes/tests/memory_code_knowledge_test/Concept.md)
- [fixture](../../functions/tests/memory_code_knowledge_test/fixture.md)
- [Scripted](../../classes/tests/memory_code_knowledge_test/Scripted.md)
- [new](../../functions/tests/memory_code_knowledge_test/Scripted/new.md)
- [connector](../../functions/tests/memory_code_knowledge_test/Scripted/connector.md)
- [remove](../../functions/tests/memory_code_knowledge_test/Scripted/remove.md)
- [reference](../../functions/tests/memory_code_knowledge_test/Scripted/reference.md)
- [tools](../../functions/tests/memory_code_knowledge_test/Scripted/okfserver/tools.md)
- [call](../../functions/tests/memory_code_knowledge_test/Scripted/okfserver/call.md)
- [Setup](../../classes/tests/memory_code_knowledge_test/Setup.md)
- [setup](../../functions/tests/memory_code_knowledge_test/setup.md)
- [save](../../functions/tests/memory_code_knowledge_test/save.md)
- [recall](../../functions/tests/memory_code_knowledge_test/recall.md)
- [concepts](../../functions/tests/memory_code_knowledge_test/concepts.md)
- [a_memory_naming_a_function_is_anchored_to_it_and_an_ambiguous_name_is_not](../../functions/tests/memory_code_knowledge_test/a_memory_naming_a_function_is_anchored_to_it_and_an_ambiguous_name_is_not.md)
- [naming_the_concept_or_a_caller_recalls_the_anchored_memory](../../functions/tests/memory_code_knowledge_test/naming_the_concept_or_a_caller_recalls_the_anchored_memory.md)
- [reading_the_file_that_defines_the_concept_brings_the_memory_once](../../functions/tests/memory_code_knowledge_test/reading_the_file_that_defines_the_concept_brings_the_memory_once.md)
- [a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one](../../functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one.md)
- [an_older_okf_mcp_turns_the_feature_off_and_recall_still_works](../../functions/tests/memory_code_knowledge_test/an_older_okf_mcp_turns_the_feature_off_and_recall_still_works.md)
- [a_slow_okf_mcp_is_skipped_and_recall_still_works](../../functions/tests/memory_code_knowledge_test/a_slow_okf_mcp_is_skipped_and_recall_still_works.md)
- [without_code_knowledge_nothing_changes](../../functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes.md)
- [real_okf_mcp_answers_in_json](../../functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json.md)

# Imports

- `async_trait::async_trait`
- `crustly::config::MemoryConfig`
- `crustly::db::Database`
- `crustly::mcp::client::McpToolDef`
- `crustly::memory::code_knowledge::{CodeKnowledge, Connector, Detection, OkfServer}`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`
- `crustly::memory::project::{id_for_root, ProjectIdentity}`
- `crustly::memory::{MemoryStore, RecallRequest}`
- `serde_json::{json, Value}`
- `std::path::{Path, PathBuf}`
- `std::sync::{Arc, Mutex}`
- `std::time::Duration`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)