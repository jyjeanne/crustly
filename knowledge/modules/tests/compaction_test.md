---
type: Rust Module
title: compaction_test
resource: tests/compaction_test.rs#L1-L171
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/crustly-db-database
  - external/crustly-llm-agent-compaction-compact
  - external/crustly-llm-agent-context-agentcontext
  - external/crustly-llm-provider-types-contentblock-message-role
  - external/uuid-uuid
  member_of:
  - packages/crustly
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