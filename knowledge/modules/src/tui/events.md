---
type: Rust Module
title: events
resource: src/tui/events.rs#L1-L848
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crate-llm-agent-agentresponse
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crossterm-event-keycode-keyevent-keymodifiers
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-mpsc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crossterm-event-poll-read
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crossterm-event-event-keyeventkind
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [TuiEvent](../../../classes/src/tui/events/TuiEvent.md)
- [ToolApprovalRequest](../../../classes/src/tui/events/ToolApprovalRequest.md)
- [is_timed_out](../../../functions/src/tui/events/ToolApprovalRequest/is_timed_out.md)
- [time_remaining](../../../functions/src/tui/events/ToolApprovalRequest/time_remaining.md)
- [ToolApprovalResponse](../../../classes/src/tui/events/ToolApprovalResponse.md)
- [AppMode](../../../classes/src/tui/events/AppMode.md)
- [EventHandler](../../../classes/src/tui/events/EventHandler.md)
- [new](../../../functions/src/tui/events/EventHandler/new.md)
- [sender](../../../functions/src/tui/events/EventHandler/sender.md)
- [try_next](../../../functions/src/tui/events/EventHandler/try_next.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)
- [start_terminal_listener](../../../functions/src/tui/events/EventHandler/start_terminal_listener.md)
- [read_terminal_events](../../../functions/src/tui/events/read_terminal_events.md)
- [coalesce_input](../../../functions/src/tui/events/coalesce_input.md)
- [flush_run](../../../functions/src/tui/events/flush_run.md)
- [pasted_text](../../../functions/src/tui/events/pasted_text.md)
- [is_text_press](../../../functions/src/tui/events/is_text_press.md)
- [typed_char](../../../functions/src/tui/events/typed_char.md)
- [default](../../../functions/src/tui/events/EventHandler/default/default.md)
- [key_matches](../../../functions/src/tui/events/key_matches.md)
- [is_quit](../../../functions/src/tui/events/is_quit.md)
- [is_new_session](../../../functions/src/tui/events/is_new_session.md)
- [is_list_sessions](../../../functions/src/tui/events/is_list_sessions.md)
- [is_help](../../../functions/src/tui/events/is_help.md)
- [is_clear_session](../../../functions/src/tui/events/is_clear_session.md)
- [is_toggle_plan](../../../functions/src/tui/events/is_toggle_plan.md)
- [is_model_download](../../../functions/src/tui/events/is_model_download.md)
- [is_model_info](../../../functions/src/tui/events/is_model_info.md)
- [is_provider_switch](../../../functions/src/tui/events/is_provider_switch.md)
- [is_llama_cpp_models](../../../functions/src/tui/events/is_llama_cpp_models.md)
- [is_copy_response](../../../functions/src/tui/events/is_copy_response.md)
- [is_paste_clipboard](../../../functions/src/tui/events/is_paste_clipboard.md)
- [is_toggle_auto_mode](../../../functions/src/tui/events/is_toggle_auto_mode.md)
- [is_submit](../../../functions/src/tui/events/is_submit.md)
- [is_newline](../../../functions/src/tui/events/is_newline.md)
- [is_cancel](../../../functions/src/tui/events/is_cancel.md)
- [is_enter](../../../functions/src/tui/events/is_enter.md)
- [is_up](../../../functions/src/tui/events/is_up.md)
- [is_down](../../../functions/src/tui/events/is_down.md)
- [is_page_up](../../../functions/src/tui/events/is_page_up.md)
- [is_page_down](../../../functions/src/tui/events/is_page_down.md)
- [is_approve](../../../functions/src/tui/events/is_approve.md)
- [is_deny](../../../functions/src/tui/events/is_deny.md)
- [is_view_details](../../../functions/src/tui/events/is_view_details.md)
- [test_event_handler_creation](../../../functions/src/tui/events/test_event_handler_creation.md)
- [test_key_matches](../../../functions/src/tui/events/test_key_matches.md)
- [test_quit_key](../../../functions/src/tui/events/test_quit_key.md)
- [test_submit_key](../../../functions/src/tui/events/test_submit_key.md)
- [test_model_info_key](../../../functions/src/tui/events/test_model_info_key.md)
- [test_provider_switch_key](../../../functions/src/tui/events/test_provider_switch_key.md)
- [test_llama_cpp_models_key](../../../functions/src/tui/events/test_llama_cpp_models_key.md)
- [test_copy_response_key](../../../functions/src/tui/events/test_copy_response_key.md)
- [test_paste_clipboard_key](../../../functions/src/tui/events/test_paste_clipboard_key.md)
- [test_toggle_auto_mode_key](../../../functions/src/tui/events/test_toggle_auto_mode_key.md)
- [test_newline_key](../../../functions/src/tui/events/test_newline_key.md)
- [press](../../../functions/src/tui/events/press.md)
- [typed](../../../functions/src/tui/events/typed.md)
- [pasted](../../../functions/src/tui/events/pasted.md)
- [keys_read_together_become_one_paste](../../../functions/src/tui/events/keys_read_together_become_one_paste.md)
- [a_single_key_stays_a_key](../../../functions/src/tui/events/a_single_key_stays_a_key.md)
- [shortcuts_and_navigation_split_a_paste_and_pass_through](../../../functions/src/tui/events/shortcuts_and_navigation_split_a_paste_and_pass_through.md)
- [releases_are_dropped_without_splitting_a_paste](../../../functions/src/tui/events/releases_are_dropped_without_splitting_a_paste.md)
- [shifted_and_altgr_characters_are_text](../../../functions/src/tui/events/shifted_and_altgr_characters_are_text.md)
- [a_crlf_pair_is_one_line_break](../../../functions/src/tui/events/a_crlf_pair_is_one_line_break.md)
- [bracketed_pastes_and_resizes_keep_their_order](../../../functions/src/tui/events/bracketed_pastes_and_resizes_keep_their_order.md)

# Imports

- `crate::llm::agent::AgentResponse`
- `crossterm::event::{KeyCode, KeyEvent, KeyModifiers}`
- `serde_json::Value`
- `std::time::Duration`
- `tokio::sync::mpsc`
- `uuid::Uuid`
- `crossterm::event::{poll, read}`
- `std::time::Instant`
- `crossterm::event::{Event, KeyEventKind}`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)