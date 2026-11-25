---
type: Rust Module
title: render
resource: src/tui/render.rs#L1-L4173
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-app-app-displaymessage
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-components-ferris
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-events-appmode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-markdown-parse-markdown-parse-plain-text
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-planexecmode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/ratatui-layout-alignment-constraint-direction-layout-rect-style-color-modifier-style-text-line-span-widgets-block-borders-paragraph-wrap-frame
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-cell-refcell
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-hash-hash-hasher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-app-visible-memories
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memoryscope-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-llama-cpp-models-hardwarefit
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
  - target: external/crate-tui-app-displaymessage
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/ratatui-backend-testbackend
    resolved_by: tree-sitter
    confidence: exact
  - target: external/ratatui-terminal
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-perfmetrics
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memory-memorycategory-memoryscope-memorysource-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [render](../../../functions/src/tui/render/render.md)
- [input_box_height](../../../functions/src/tui/render/input_box_height.md)
- [tail_chars](../../../functions/src/tui/render/tail_chars.md)
- [MessageLineCache](../../../classes/src/tui/render/MessageLineCache.md)
- [lines_for](../../../functions/src/tui/render/MessageLineCache/lines_for.md)
- [retain](../../../functions/src/tui/render/MessageLineCache/retain.md)
- [render_key](../../../functions/src/tui/render/render_key.md)
- [provider_icon](../../../functions/src/tui/render/provider_icon.md)
- [short_token_count](../../../functions/src/tui/render/short_token_count.md)
- [context_usage_color](../../../functions/src/tui/render/context_usage_color.md)
- [render_header](../../../functions/src/tui/render/render_header.md)
- [render_pending_plan_banner](../../../functions/src/tui/render/render_pending_plan_banner.md)
- [render_perf_footer](../../../functions/src/tui/render/render_perf_footer.md)
- [render_thinking_block](../../../functions/src/tui/render/render_thinking_block.md)
- [render_message_lines](../../../functions/src/tui/render/render_message_lines.md)
- [render_streaming_response](../../../functions/src/tui/render/render_streaming_response.md)
- [format_elapsed](../../../functions/src/tui/render/format_elapsed.md)
- [render_processing_indicator](../../../functions/src/tui/render/render_processing_indicator.md)
- [chat_block](../../../functions/src/tui/render/chat_block.md)
- [fit_scroll](../../../functions/src/tui/render/fit_scroll.md)
- [render_chat](../../../functions/src/tui/render/render_chat.md)
- [render_welcome](../../../functions/src/tui/render/render_welcome.md)
- [render_input](../../../functions/src/tui/render/render_input.md)
- [render_sessions](../../../functions/src/tui/render/render_sessions.md)
- [render_skills](../../../functions/src/tui/render/render_skills.md)
- [render_memory](../../../functions/src/tui/render/render_memory.md)
- [one_line](../../../functions/src/tui/render/one_line.md)
- [render_mcp](../../../functions/src/tui/render/render_mcp.md)
- [help_row](../../../functions/src/tui/render/help_row.md)
- [feature_row](../../../functions/src/tui/render/feature_row.md)
- [help_section_header](../../../functions/src/tui/render/help_section_header.md)
- [help_global_commands](../../../functions/src/tui/render/help_global_commands.md)
- [help_chat_mode](../../../functions/src/tui/render/help_chat_mode.md)
- [help_session_list](../../../functions/src/tui/render/help_session_list.md)
- [help_plan_mode](../../../functions/src/tui/render/help_plan_mode.md)
- [help_features](../../../functions/src/tui/render/help_features.md)
- [help_about](../../../functions/src/tui/render/help_about.md)
- [help_footer](../../../functions/src/tui/render/help_footer.md)
- [render_help](../../../functions/src/tui/render/render_help.md)
- [render_plan_help](../../../functions/src/tui/render/render_plan_help.md)
- [render_plan_task_lines](../../../functions/src/tui/render/render_plan_task_lines.md)
- [render_plan_document](../../../functions/src/tui/render/render_plan_document.md)
- [render_plan_empty_state](../../../functions/src/tui/render/render_plan_empty_state.md)
- [render_plan](../../../functions/src/tui/render/render_plan.md)
- [render_settings](../../../functions/src/tui/render/render_settings.md)
- [approval_dialog_area](../../../functions/src/tui/render/approval_dialog_area.md)
- [render_approval_header](../../../functions/src/tui/render/render_approval_header.md)
- [render_approval_capabilities](../../../functions/src/tui/render/render_approval_capabilities.md)
- [render_approval_preview](../../../functions/src/tui/render/render_approval_preview.md)
- [render_approval_input_detailed](../../../functions/src/tui/render/render_approval_input_detailed.md)
- [render_approval_input_summary](../../../functions/src/tui/render/render_approval_input_summary.md)
- [render_approval_actions](../../../functions/src/tui/render/render_approval_actions.md)
- [render_approval](../../../functions/src/tui/render/render_approval.md)
- [render_file_picker](../../../functions/src/tui/render/render_file_picker.md)
- [render_model_info](../../../functions/src/tui/render/render_model_info.md)
- [render_provider_switch](../../../functions/src/tui/render/render_provider_switch.md)
- [render_model_download](../../../functions/src/tui/render/render_model_download.md)
- [render_model_download_progress](../../../functions/src/tui/render/render_model_download_progress.md)
- [render_model_download_confirm_delete](../../../functions/src/tui/render/render_model_download_confirm_delete.md)
- [render_model_download_deleting](../../../functions/src/tui/render/render_model_download_deleting.md)
- [format_param_count](../../../functions/src/tui/render/format_param_count.md)
- [llama_cpp_fit_tag](../../../functions/src/tui/render/llama_cpp_fit_tag.md)
- [llama_cpp_fit_tag](../../../functions/src/tui/render/llama_cpp_fit_tag-2.md)
- [llama_cpp_model_lines](../../../functions/src/tui/render/llama_cpp_model_lines.md)
- [llama_cpp_hardware_line](../../../functions/src/tui/render/llama_cpp_hardware_line.md)
- [render_llama_cpp_models](../../../functions/src/tui/render/render_llama_cpp_models.md)
- [render_llama_cpp_download_progress](../../../functions/src/tui/render/render_llama_cpp_download_progress.md)
- [render_llama_cpp_confirm_delete](../../../functions/src/tui/render/render_llama_cpp_confirm_delete.md)
- [render_llama_cpp_deleting](../../../functions/src/tui/render/render_llama_cpp_deleting.md)
- [render_llama_cpp_switching](../../../functions/src/tui/render/render_llama_cpp_switching.md)
- [render_status_bar](../../../functions/src/tui/render/render_status_bar.md)
- [DummyProvider](../../../classes/src/tui/render/DummyProvider.md)
- [complete](../../../functions/src/tui/render/DummyProvider/provider/complete.md)
- [stream](../../../functions/src/tui/render/DummyProvider/provider/stream.md)
- [name](../../../functions/src/tui/render/DummyProvider/provider/name.md)
- [default_model](../../../functions/src/tui/render/DummyProvider/provider/default_model.md)
- [supported_models](../../../functions/src/tui/render/DummyProvider/provider/supported_models.md)
- [context_window](../../../functions/src/tui/render/DummyProvider/provider/context_window.md)
- [calculate_cost](../../../functions/src/tui/render/DummyProvider/provider/calculate_cost.md)
- [test_app](../../../functions/src/tui/render/test_app.md)
- [render_to_string](../../../functions/src/tui/render/render_to_string.md)
- [line_text](../../../functions/src/tui/render/line_text.md)
- [message_header_timestamp_is_shown_in_local_time](../../../functions/src/tui/render/message_header_timestamp_is_shown_in_local_time.md)
- [header_shows_ollama_provider_badge_and_tokens_per_second](../../../functions/src/tui/render/header_shows_ollama_provider_badge_and_tokens_per_second.md)
- [header_omits_tokens_per_second_when_unavailable](../../../functions/src/tui/render/header_omits_tokens_per_second_when_unavailable.md)
- [status_bar_shows_interactive_by_default](../../../functions/src/tui/render/status_bar_shows_interactive_by_default.md)
- [status_bar_shows_full_auto_when_active](../../../functions/src/tui/render/status_bar_shows_full_auto_when_active.md)
- [skills_view_shows_name_and_description](../../../functions/src/tui/render/skills_view_shows_name_and_description.md)
- [skills_view_shows_empty_state_message](../../../functions/src/tui/render/skills_view_shows_empty_state_message.md)
- [mcp_view_shows_connected_server_with_tool_count](../../../functions/src/tui/render/mcp_view_shows_connected_server_with_tool_count.md)
- [mcp_view_shows_connection_error](../../../functions/src/tui/render/mcp_view_shows_connection_error.md)
- [mcp_view_shows_empty_state_message](../../../functions/src/tui/render/mcp_view_shows_empty_state_message.md)
- [model_download_dialog_shows_prompt_and_suggestions](../../../functions/src/tui/render/model_download_dialog_shows_prompt_and_suggestions.md)
- [model_download_progress_shows_status_and_bar](../../../functions/src/tui/render/model_download_progress_shows_status_and_bar.md)
- [model_download_confirm_delete_shows_prompt](../../../functions/src/tui/render/model_download_confirm_delete_shows_prompt.md)
- [model_download_deleting_shows_status](../../../functions/src/tui/render/model_download_deleting_shows_status.md)
- [llama_cpp_models_dialog_shows_prompt_and_local_models](../../../functions/src/tui/render/llama_cpp_models_dialog_shows_prompt_and_local_models.md)
- [llama_cpp_model_fixture](../../../functions/src/tui/render/llama_cpp_model_fixture.md)
- [format_param_count_formats_at_the_expected_scale](../../../functions/src/tui/render/format_param_count_formats_at_the_expected_scale.md)
- [llama_cpp_model_lines_prefers_display_name_over_the_raw_filename](../../../functions/src/tui/render/llama_cpp_model_lines_prefers_display_name_over_the_raw_filename.md)
- [llama_cpp_model_lines_labels_a_paired_and_an_unpaired_mmproj_entry_differently](../../../functions/src/tui/render/llama_cpp_model_lines_labels_a_paired_and_an_unpaired_mmproj_entry_differently.md)
- [llama_cpp_model_lines_shows_a_detail_line_only_when_selected](../../../functions/src/tui/render/llama_cpp_model_lines_shows_a_detail_line_only_when_selected.md)
- [llama_cpp_model_lines_shows_a_fit_tag_when_budget_is_known](../../../functions/src/tui/render/llama_cpp_model_lines_shows_a_fit_tag_when_budget_is_known.md)
- [llama_cpp_model_lines_shows_wont_fit_when_estimate_exceeds_budget](../../../functions/src/tui/render/llama_cpp_model_lines_shows_wont_fit_when_estimate_exceeds_budget.md)
- [llama_cpp_model_lines_shows_the_context_length_the_estimate_used](../../../functions/src/tui/render/llama_cpp_model_lines_shows_the_context_length_the_estimate_used.md)
- [llama_cpp_model_lines_shows_no_fit_tag_without_the_gguf_management_feature](../../../functions/src/tui/render/llama_cpp_model_lines_shows_no_fit_tag_without_the_gguf_management_feature.md)
- [llama_cpp_model_lines_shows_no_fit_tag_when_budget_is_unknown](../../../functions/src/tui/render/llama_cpp_model_lines_shows_no_fit_tag_when_budget_is_unknown.md)
- [llama_cpp_models_dialog_shows_hardware_detecting_state](../../../functions/src/tui/render/llama_cpp_models_dialog_shows_hardware_detecting_state.md)
- [llama_cpp_models_dialog_shows_hardware_summary_once_detected](../../../functions/src/tui/render/llama_cpp_models_dialog_shows_hardware_summary_once_detected.md)
- [llama_cpp_models_dialog_shows_display_name_not_the_raw_path](../../../functions/src/tui/render/llama_cpp_models_dialog_shows_display_name_not_the_raw_path.md)
- [llama_cpp_models_dialog_shows_loading_state](../../../functions/src/tui/render/llama_cpp_models_dialog_shows_loading_state.md)
- [llama_cpp_download_progress_shows_status_and_bar](../../../functions/src/tui/render/llama_cpp_download_progress_shows_status_and_bar.md)
- [llama_cpp_confirm_delete_shows_prompt](../../../functions/src/tui/render/llama_cpp_confirm_delete_shows_prompt.md)
- [llama_cpp_switching_shows_loading_message](../../../functions/src/tui/render/llama_cpp_switching_shows_loading_message.md)
- [chat_input_renders_textarea_contents_and_hint](../../../functions/src/tui/render/chat_input_renders_textarea_contents_and_hint.md)
- [model_info_panel_shows_provider_model_and_context_window](../../../functions/src/tui/render/model_info_panel_shows_provider_model_and_context_window.md)
- [model_info_panel_shows_last_response_perf_metrics](../../../functions/src/tui/render/model_info_panel_shows_last_response_perf_metrics.md)
- [help_screen_lists_commands_from_every_section](../../../functions/src/tui/render/help_screen_lists_commands_from_every_section.md)
- [help_screen_shows_shift_enter_when_kitty_protocol_active](../../../functions/src/tui/render/help_screen_shows_shift_enter_when_kitty_protocol_active.md)
- [chat_shows_pending_plan_banner_only_while_awaiting_approval](../../../functions/src/tui/render/chat_shows_pending_plan_banner_only_while_awaiting_approval.md)
- [chat_message_thinking_block_toggles_between_collapsed_and_expanded](../../../functions/src/tui/render/chat_message_thinking_block_toggles_between_collapsed_and_expanded.md)
- [chat_message_perf_footer_reports_cold_and_warm_starts](../../../functions/src/tui/render/chat_message_perf_footer_reports_cold_and_warm_starts.md)
- [chat_shows_streaming_response_and_processing_indicator](../../../functions/src/tui/render/chat_shows_streaming_response_and_processing_indicator.md)
- [assistant_message](../../../functions/src/tui/render/assistant_message.md)
- [chat_follows_the_end_of_a_reply_whose_paragraphs_wrap](../../../functions/src/tui/render/chat_follows_the_end_of_a_reply_whose_paragraphs_wrap.md)
- [scrolling_up_hides_the_end_and_says_how_to_return](../../../functions/src/tui/render/scrolling_up_hides_the_end_and_says_how_to_return.md)
- [an_empty_chat_shows_ferris_and_a_welcome](../../../functions/src/tui/render/an_empty_chat_shows_ferris_and_a_welcome.md)
- [ferris_works_while_the_model_thinks](../../../functions/src/tui/render/ferris_works_while_the_model_thinks.md)
- [the_input_box_grows_with_its_text](../../../functions/src/tui/render/the_input_box_grows_with_its_text.md)
- [long_paths_are_cut_on_a_character_boundary](../../../functions/src/tui/render/long_paths_are_cut_on_a_character_boundary.md)
- [elapsed_time_reads_naturally](../../../functions/src/tui/render/elapsed_time_reads_naturally.md)
- [a_new_session_costs_zero_not_minus_zero](../../../functions/src/tui/render/a_new_session_costs_zero_not_minus_zero.md)
- [plan_mode_shows_full_document_with_tasks_and_criteria](../../../functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria.md)
- [plan_mode_shows_empty_state_without_a_plan](../../../functions/src/tui/render/plan_mode_shows_empty_state_without_a_plan.md)
- [test_approval_request](../../../functions/src/tui/render/test_approval_request.md)
- [approval_dialog_shows_tool_name_capabilities_and_summarized_params](../../../functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params.md)
- [approval_dialog_details_view_shows_pretty_printed_json](../../../functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json.md)
- [the_memories_screen_shows_its_filters_and_the_memory](../../../functions/src/tui/render/the_memories_screen_shows_its_filters_and_the_memory.md)
- [the_memories_screen_scrolls_to_the_selection](../../../functions/src/tui/render/the_memories_screen_scrolls_to_the_selection.md)

# Imports

- `super::app::{App, DisplayMessage}`
- `super::components::ferris`
- `super::events::AppMode`
- `super::markdown::{parse_markdown, parse_plain_text}`
- `crate::config::PlanExecMode`
- `ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
}`
- `std::cell::RefCell`
- `std::collections::{HashMap, HashSet}`
- `std::hash::{Hash, Hasher}`
- `uuid::Uuid`
- `super::app::visible_memories`
- `crate::memory::model::{MemoryScope, MemoryStatus}`
- `crate::llm::provider::llama_cpp_models::HardwareFit`
- `super::*`
- `crate::db::Database`
- `crate::llm::agent::AgentService`
- `crate::llm::provider::{
        LLMRequest, LLMResponse, Provider, ProviderStream, Result as ProviderResult,
    }`
- `crate::services::ServiceContext`
- `crate::tui::app::DisplayMessage`
- `async_trait::async_trait`
- `ratatui::backend::TestBackend`
- `ratatui::Terminal`
- `std::sync::Arc`
- `crate::llm::provider::PerfMetrics`
- `std::time::Duration`
- `crate::memory::model::{
            Memory, MemoryCategory, MemoryScope, MemorySource, MemoryStatus,
        }`

# Member of

- [crustly](../../../packages/crustly.md)