---
type: Rust Module
title: context
resource: src/llm/agent/context.rs#L1-L357
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/crate-db-models-message-as-dbmessage
  - external/crate-llm-provider-types-cachemetrics
  - external/crate-llm-provider-contentblock-message-role
  - external/std-path-pathbuf
  - external/uuid-uuid
  - external/crate-db-repository-episodicmemoryrepository
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [AgentContext](../../../../classes/src/llm/agent/context/AgentContext.md)
- [TrackedFile](../../../../classes/src/llm/agent/context/TrackedFile.md)
- [new](../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [with_system_prompt](../../../../functions/src/llm/agent/context/AgentContext/with_system_prompt.md)
- [add_message](../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [from_db_messages](../../../../functions/src/llm/agent/context/AgentContext/from_db_messages.md)
- [track_file](../../../../functions/src/llm/agent/context/AgentContext/track_file.md)
- [would_exceed_limit](../../../../functions/src/llm/agent/context/AgentContext/would_exceed_limit.md)
- [estimate_message_tokens](../../../../functions/src/llm/agent/context/AgentContext/estimate_message_tokens.md)
- [estimate_tokens](../../../../functions/src/llm/agent/context/AgentContext/estimate_tokens.md)
- [usage_percentage](../../../../functions/src/llm/agent/context/AgentContext/usage_percentage.md)
- [should_compact](../../../../functions/src/llm/agent/context/AgentContext/should_compact.md)
- [trim_to_fit](../../../../functions/src/llm/agent/context/AgentContext/trim_to_fit.md)
- [inject_episodic_memories](../../../../functions/src/llm/agent/context/AgentContext/inject_episodic_memories.md)
- [token_count](../../../../functions/src/llm/agent/context/token_count.md)
- [test_context_creation](../../../../functions/src/llm/agent/context/test_context_creation.md)
- [test_add_message](../../../../functions/src/llm/agent/context/test_add_message.md)
- [test_system_prompt](../../../../functions/src/llm/agent/context/test_system_prompt.md)
- [test_token_estimation](../../../../functions/src/llm/agent/context/test_token_estimation.md)
- [test_would_exceed_limit](../../../../functions/src/llm/agent/context/test_would_exceed_limit.md)
- [test_usage_percentage](../../../../functions/src/llm/agent/context/test_usage_percentage.md)
- [test_trim_to_fit](../../../../functions/src/llm/agent/context/test_trim_to_fit.md)
- [token_count_bpe_accuracy_rust_file](../../../../functions/src/llm/agent/context/token_count_bpe_accuracy_rust_file.md)
- [token_count_empty_string](../../../../functions/src/llm/agent/context/token_count_empty_string.md)
- [token_count_prose_reasonable](../../../../functions/src/llm/agent/context/token_count_prose_reasonable.md)

# Imports

- `crate::db::models::Message as DbMessage`
- `crate::llm::provider::types::CacheMetrics`
- `crate::llm::provider::{ContentBlock, Message, Role}`
- `std::path::PathBuf`
- `uuid::Uuid`
- `crate::db::repository::EpisodicMemoryRepository`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)