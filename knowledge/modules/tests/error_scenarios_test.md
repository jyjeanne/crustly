---
type: Rust Module
title: error_scenarios_test
resource: tests/error_scenarios_test.rs#L1-L362
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/anyhow-result
  - external/async-trait-async-trait
  - external/crustly-db-database-llm-agent-agentservice-provider-error-providererror-result-as-providerresult-types-contentblock-llmrequest-llmresponse-stopreason-tokenusage-provider-providerstream-tools-bash-bashtool-read-readtool-registry-toolregistry-write-writetool-services-servicecontext-sessionservice
  - external/std-sync-arc
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [ErrorMockProvider](../../classes/tests/error_scenarios_test/ErrorMockProvider.md)
- [ErrorType](../../classes/tests/error_scenarios_test/ErrorType.md)
- [new](../../functions/tests/error_scenarios_test/ErrorMockProvider/new.md)
- [complete](../../functions/tests/error_scenarios_test/ErrorMockProvider/provider/complete.md)
- [stream](../../functions/tests/error_scenarios_test/ErrorMockProvider/provider/stream.md)
- [name](../../functions/tests/error_scenarios_test/ErrorMockProvider/provider/name.md)
- [default_model](../../functions/tests/error_scenarios_test/ErrorMockProvider/provider/default_model.md)
- [supported_models](../../functions/tests/error_scenarios_test/ErrorMockProvider/provider/supported_models.md)
- [context_window](../../functions/tests/error_scenarios_test/ErrorMockProvider/provider/context_window.md)
- [calculate_cost](../../functions/tests/error_scenarios_test/ErrorMockProvider/provider/calculate_cost.md)
- [create_test_db](../../functions/tests/error_scenarios_test/create_test_db.md)
- [create_error_agent](../../functions/tests/error_scenarios_test/create_error_agent.md)
- [test_error_api_error](../../functions/tests/error_scenarios_test/test_error_api_error.md)
- [test_error_rate_limit](../../functions/tests/error_scenarios_test/test_error_rate_limit.md)
- [test_error_timeout](../../functions/tests/error_scenarios_test/test_error_timeout.md)
- [test_error_invalid_response](../../functions/tests/error_scenarios_test/test_error_invalid_response.md)
- [test_error_authentication](../../functions/tests/error_scenarios_test/test_error_authentication.md)
- [test_error_session_not_found](../../functions/tests/error_scenarios_test/test_error_session_not_found.md)
- [test_error_empty_message](../../functions/tests/error_scenarios_test/test_error_empty_message.md)
- [test_error_database_concurrent_access](../../functions/tests/error_scenarios_test/test_error_database_concurrent_access.md)
- [test_error_recovery_after_failure](../../functions/tests/error_scenarios_test/test_error_recovery_after_failure.md)
- [WorkingMockProvider](../../classes/tests/error_scenarios_test/WorkingMockProvider.md)
- [complete](../../functions/tests/error_scenarios_test/WorkingMockProvider/provider/complete.md)
- [stream](../../functions/tests/error_scenarios_test/WorkingMockProvider/provider/stream.md)
- [name](../../functions/tests/error_scenarios_test/WorkingMockProvider/provider/name.md)
- [default_model](../../functions/tests/error_scenarios_test/WorkingMockProvider/provider/default_model.md)
- [supported_models](../../functions/tests/error_scenarios_test/WorkingMockProvider/provider/supported_models.md)
- [context_window](../../functions/tests/error_scenarios_test/WorkingMockProvider/provider/context_window.md)
- [calculate_cost](../../functions/tests/error_scenarios_test/WorkingMockProvider/provider/calculate_cost.md)

# Imports

- `anyhow::Result`
- `async_trait::async_trait`
- `crustly::{
    db::Database,
    llm::{
        agent::AgentService,
        provider::{
            error::{ProviderError, Result as ProviderResult},
            types::{ContentBlock, LLMRequest, LLMResponse, StopReason, TokenUsage},
            Provider, ProviderStream,
        },
        tools::{bash::BashTool, read::ReadTool, registry::ToolRegistry, write::WriteTool},
    },
    services::{ServiceContext, SessionService},
}`
- `std::sync::Arc`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)