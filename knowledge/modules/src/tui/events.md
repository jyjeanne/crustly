---
type: Rust Module
title: events
resource: src/tui/events.rs#L1-L548
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/crate-llm-agent-agentresponse
  - external/crossterm-event-keycode-keyevent-keymodifiers
  - external/serde-json-value
  - external/std-time-duration
  - external/tokio-sync-mpsc
  - external/uuid-uuid
  - external/super
  member_of:
  - packages/crustly
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
- [test_copy_response_key](../../../functions/src/tui/events/test_copy_response_key.md)
- [test_paste_clipboard_key](../../../functions/src/tui/events/test_paste_clipboard_key.md)
- [test_toggle_auto_mode_key](../../../functions/src/tui/events/test_toggle_auto_mode_key.md)
- [test_newline_key](../../../functions/src/tui/events/test_newline_key.md)

# Imports

- `crate::llm::agent::AgentResponse`
- `crossterm::event::{KeyCode, KeyEvent, KeyModifiers}`
- `serde_json::Value`
- `std::time::Duration`
- `tokio::sync::mpsc`
- `uuid::Uuid`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)