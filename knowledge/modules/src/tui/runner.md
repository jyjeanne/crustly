---
type: Rust Module
title: runner
resource: src/tui/runner.rs#L1-L208
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-app-app
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-events-eventhandler
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-render
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crossterm-event-disablebracketedpaste-enablebracketedpaste-keyboardenhancementflags-popkeyboardenhancementflags-pushkeyboardenhancementflags-execute-terminal-disable-raw-mode-enable-raw-mode-supports-keyboard-enhancement-enteralternatescreen-leavealternatescreen
    resolved_by: tree-sitter
    confidence: exact
  - target: external/ratatui-backend-backend-crosstermbackend-terminal
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-agentservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-llmrequest-llmresponse-provider-providerstream-result-as-providerresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-services-servicecontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/ratatui-backend-testbackend
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
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