---
type: Rust Module
title: compaction_test
resource: tests/compaction_test.rs#L1-L174
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-llm-agent-compaction-compact
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-llm-agent-context-agentcontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-llm-provider-types-contentblock-message-role
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

- [create_session](../../functions/tests/compaction_test/create_session.md)
- [text_message](../../functions/tests/compaction_test/text_message.md)
- [build_context](../../functions/tests/compaction_test/build_context.md)
- [compaction_preserves_last_10_turns](../../functions/tests/compaction_test/compaction_preserves_last_10_turns.md)
- [compaction_fails_gracefully_with_insufficient_turns](../../functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns.md)
- [should_compact_fires_at_80_percent](../../functions/tests/compaction_test/should_compact_fires_at_80_percent.md)
- [compaction_writes_one_record_to_db](../../functions/tests/compaction_test/compaction_writes_one_record_to_db.md)

# Imports

- `crustly::db::Database`
- `crustly::llm::agent::compaction::compact`
- `crustly::llm::agent::context::AgentContext`
- `crustly::llm::provider::types::{ContentBlock, Message, Role}`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)