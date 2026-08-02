---
type: Rust Method
title: handle_key_event
resource: src/tui/app.rs#L1008-L1123
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/is_quit
  - functions/src/tui/events/is_new_session
  - functions/src/tui/app/App/create_new_session
  - functions/src/tui/events/is_list_sessions
  - functions/src/tui/app/App/switch_mode
  - functions/src/tui/events/is_help
  - functions/src/tui/events/is_clear_session
  - functions/src/tui/app/App/clear_session
  - functions/src/tui/events/is_toggle_plan
  - functions/src/tui/app/App/load_plan_for_viewing
  - functions/src/tui/events/is_toggle_auto_mode
  - functions/src/tui/app/App/cycle_auto_mode
  - functions/src/tui/events/is_model_download
  - functions/src/tui/app/App/open_model_download
  - functions/src/tui/events/is_model_info
  - functions/src/tui/events/is_provider_switch
  - functions/src/tui/app/App/open_provider_switch
  - functions/src/tui/events/is_llama_cpp_models
  - functions/src/tui/app/App/open_llama_cpp_models
  - functions/src/tui/app/App/handle_chat_key
  - functions/src/tui/app/App/handle_plan_key
  - functions/src/tui/app/App/handle_sessions_key
  - functions/src/tui/app/App/handle_approval_key
  - functions/src/tui/app/App/handle_file_picker_key
  - functions/src/tui/app/App/handle_model_download_key
  - functions/src/tui/app/App/handle_provider_switch_key
  - functions/src/tui/app/App/handle_llama_cpp_models_key
  - functions/src/tui/app/App/handle_skills_key
  - functions/src/tui/app/App/handle_mcp_key
  - functions/src/tui/events/is_cancel
  called_by:
  - functions/src/tui/app/App/handle_event
  - functions/src/tui/app/shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps
  - functions/src/tui/app/shift_tab_works_from_any_mode_not_just_chat
  - functions/src/tui/app/setting_auto_mode_state_shares_the_same_cell_as_a_clone
  - functions/src/tui/app/ctrl_o_opens_model_info_panel_and_esc_closes_it
  - functions/src/tui/app/ctrl_w_opens_provider_switch_dialog_in_loading_state
  - functions/src/tui/app/ctrl_g_opens_llama_cpp_models_dialog_in_loading_state
---

# Signature

`async fn handle_key_event(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_quit](../../../../../functions/src/tui/events/is_quit.md)
- [is_new_session](../../../../../functions/src/tui/events/is_new_session.md)
- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [is_list_sessions](../../../../../functions/src/tui/events/is_list_sessions.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [is_help](../../../../../functions/src/tui/events/is_help.md)
- [is_clear_session](../../../../../functions/src/tui/events/is_clear_session.md)
- [clear_session](../../../../../functions/src/tui/app/App/clear_session.md)
- [is_toggle_plan](../../../../../functions/src/tui/events/is_toggle_plan.md)
- [load_plan_for_viewing](../../../../../functions/src/tui/app/App/load_plan_for_viewing.md)
- [is_toggle_auto_mode](../../../../../functions/src/tui/events/is_toggle_auto_mode.md)
- [cycle_auto_mode](../../../../../functions/src/tui/app/App/cycle_auto_mode.md)
- [is_model_download](../../../../../functions/src/tui/events/is_model_download.md)
- [open_model_download](../../../../../functions/src/tui/app/App/open_model_download.md)
- [is_model_info](../../../../../functions/src/tui/events/is_model_info.md)
- [is_provider_switch](../../../../../functions/src/tui/events/is_provider_switch.md)
- [open_provider_switch](../../../../../functions/src/tui/app/App/open_provider_switch.md)
- [is_llama_cpp_models](../../../../../functions/src/tui/events/is_llama_cpp_models.md)
- [open_llama_cpp_models](../../../../../functions/src/tui/app/App/open_llama_cpp_models.md)
- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)
- [handle_sessions_key](../../../../../functions/src/tui/app/App/handle_sessions_key.md)
- [handle_approval_key](../../../../../functions/src/tui/app/App/handle_approval_key.md)
- [handle_file_picker_key](../../../../../functions/src/tui/app/App/handle_file_picker_key.md)
- [handle_model_download_key](../../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [handle_provider_switch_key](../../../../../functions/src/tui/app/App/handle_provider_switch_key.md)
- [handle_llama_cpp_models_key](../../../../../functions/src/tui/app/App/handle_llama_cpp_models_key.md)
- [handle_skills_key](../../../../../functions/src/tui/app/App/handle_skills_key.md)
- [handle_mcp_key](../../../../../functions/src/tui/app/App/handle_mcp_key.md)
- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps](../../../../../functions/src/tui/app/shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps.md)
- [shift_tab_works_from_any_mode_not_just_chat](../../../../../functions/src/tui/app/shift_tab_works_from_any_mode_not_just_chat.md)
- [setting_auto_mode_state_shares_the_same_cell_as_a_clone](../../../../../functions/src/tui/app/setting_auto_mode_state_shares_the_same_cell_as_a_clone.md)
- [ctrl_o_opens_model_info_panel_and_esc_closes_it](../../../../../functions/src/tui/app/ctrl_o_opens_model_info_panel_and_esc_closes_it.md)
- [ctrl_w_opens_provider_switch_dialog_in_loading_state](../../../../../functions/src/tui/app/ctrl_w_opens_provider_switch_dialog_in_loading_state.md)
- [ctrl_g_opens_llama_cpp_models_dialog_in_loading_state](../../../../../functions/src/tui/app/ctrl_g_opens_llama_cpp_models_dialog_in_loading_state.md)