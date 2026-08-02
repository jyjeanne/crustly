---
type: Rust Module
title: runner
resource: src/tui/runner.rs#L1-L203
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-app-app
  - external/super-events-eventhandler
  - external/super-render
  - external/anyhow-result
  - external/crossterm-event-disablebracketedpaste-enablebracketedpaste-keyboardenhancementflags-popkeyboardenhancementflags-pushkeyboardenhancementflags-execute-terminal-disable-raw-mode-enable-raw-mode-supports-keyboard-enhancement-enteralternatescreen-leavealternatescreen
  - external/ratatui-backend-backend-crosstermbackend-terminal
  - external/std-io
  - external/super
  - external/crate-db-database
  - external/crate-llm-agent-agentservice
  - external/crate-llm-provider-llmrequest-llmresponse-provider-providerstream-result-as-providerresult
  - external/crate-services-servicecontext
  - external/async-trait-async-trait
  - external/ratatui-backend-testbackend
  - external/std-sync-arc
  member_of:
  - packages/crustly
---

# Contains

- [run](../../../functions/src/tui/runner/run.md)
- [run_inner](../../../functions/src/tui/runner/run_inner.md)
- [run_loop](../../../functions/src/tui/runner/run_loop.md)
- [DummyProvider](../../../classes/src/tui/runner/DummyProvider.md)
- [complete](../../../functions/src/tui/runner/DummyProvider/provider/complete.md)
- [stream](../../../functions/src/tui/runner/DummyProvider/provider/stream.md)
- [name](../../../functions/src/tui/runner/DummyProvider/provider/name.md)
- [default_model](../../../functions/src/tui/runner/DummyProvider/provider/default_model.md)
- [supported_models](../../../functions/src/tui/runner/DummyProvider/provider/supported_models.md)
- [context_window](../../../functions/src/tui/runner/DummyProvider/provider/context_window.md)
- [calculate_cost](../../../functions/src/tui/runner/DummyProvider/provider/calculate_cost.md)
- [run_loop_exits_immediately_when_should_quit_is_set](../../../functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set.md)

# Imports

- `super::app::App`
- `super::events::EventHandler`
- `super::render`
- `anyhow::Result`
- `crossterm::{
    event::{
        DisableBracketedPaste, EnableBracketedPaste, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, supports_keyboard_enhancement, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
}`
- `ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
}`
- `std::io`
- `super::*`
- `crate::db::Database`
- `crate::llm::agent::AgentService`
- `crate::llm::provider::{
        LLMRequest, LLMResponse, Provider, ProviderStream, Result as ProviderResult,
    }`
- `crate::services::ServiceContext`
- `async_trait::async_trait`
- `ratatui::backend::TestBackend`
- `std::sync::Arc`

# Member of

- [crustly](../../../packages/crustly.md)