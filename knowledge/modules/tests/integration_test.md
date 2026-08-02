---
type: Rust Module
title: integration_test
resource: tests/integration_test.rs#L1-L463
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/anyhow-result
  - external/async-trait-async-trait
  - external/crustly-config-config-db-database-llm-agent-agentservice-provider-types-contentblock-llmrequest-llmresponse-stopreason-tokenusage-provider-providerstream-tools-bash-bashtool-read-readtool-registry-toolregistry-write-writetool-services-messageservice-servicecontext-sessionservice
  - external/std-sync-arc
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [MockProvider](../../classes/tests/integration_test/MockProvider.md)
- [new](../../functions/tests/integration_test/MockProvider/new.md)
- [single_response](../../functions/tests/integration_test/MockProvider/single_response.md)
- [complete](../../functions/tests/integration_test/MockProvider/provider/complete.md)
- [stream](../../functions/tests/integration_test/MockProvider/provider/stream.md)
- [name](../../functions/tests/integration_test/MockProvider/provider/name.md)
- [supports_streaming](../../functions/tests/integration_test/MockProvider/provider/supports_streaming.md)
- [calculate_cost](../../functions/tests/integration_test/MockProvider/provider/calculate_cost.md)
- [default_model](../../functions/tests/integration_test/MockProvider/provider/default_model.md)
- [supported_models](../../functions/tests/integration_test/MockProvider/provider/supported_models.md)
- [context_window](../../functions/tests/integration_test/MockProvider/provider/context_window.md)
- [create_test_db](../../functions/tests/integration_test/create_test_db.md)
- [create_test_agent](../../functions/tests/integration_test/create_test_agent.md)
- [test_end_to_end_simple_message](../../functions/tests/integration_test/test_end_to_end_simple_message.md)
- [test_end_to_end_multi_turn_conversation](../../functions/tests/integration_test/test_end_to_end_multi_turn_conversation.md)
- [test_end_to_end_session_management](../../functions/tests/integration_test/test_end_to_end_session_management.md)
- [test_end_to_end_cost_tracking](../../functions/tests/integration_test/test_end_to_end_cost_tracking.md)
- [test_end_to_end_error_handling](../../functions/tests/integration_test/test_end_to_end_error_handling.md)
- [test_end_to_end_token_usage](../../functions/tests/integration_test/test_end_to_end_token_usage.md)
- [test_end_to_end_system_prompt](../../functions/tests/integration_test/test_end_to_end_system_prompt.md)
- [test_config_loading](../../functions/tests/integration_test/test_config_loading.md)
- [test_database_persistence](../../functions/tests/integration_test/test_database_persistence.md)

# Imports

- `anyhow::Result`
- `async_trait::async_trait`
- `crustly::{
    config::Config,
    db::Database,
    llm::{
        agent::AgentService,
        provider::{
            types::{ContentBlock, LLMRequest, LLMResponse, StopReason, TokenUsage},
            Provider, ProviderStream,
        },
        tools::{bash::BashTool, read::ReadTool, registry::ToolRegistry, write::WriteTool},
    },
    services::{MessageService, ServiceContext, SessionService},
}`
- `std::sync::Arc`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)