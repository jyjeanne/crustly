# Graph Report - crustly  (2026-08-02)

## Corpus Check
- 2927 files · ~943,874 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 14464 nodes · 24082 edges · 680 communities (432 shown, 248 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 58 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `644460da`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- FileService
- SessionRepository
- service.rs
- events.rs
- openai.rs
- ollama.rs
- Provider
- render.rs
- AgentContext
- File
- app.rs
- PermissionPolicy
- PlanRepository
- SecretString
- ModelRouter
- .new
- plan_tool_security_tests.rs
- highlight.rs
- task.rs
- skill.rs
- doc_parser.rs
- todo_write.rs
- powershell.rs
- ToolRegistry
- utils/retry.rs
- logging.rs
- client.rs
- Config
- ErrorInfo
- apply_patch.rs
- error_scenarios_test.rs
- ollama_models.rs
- QwenProvider
- config/mod.rs
- Functions
- MockProvider
- Self
- ContextStore
- StreamingMockProvider
- AnthropicProvider
- LLMRequest
- .execute
- ToolExecutionContext
- db/retry.rs
- App
- CrabraceIntegration
- ToolResultCache
- MessageService
- .local
- retry_with_backoff
- bash.rs
- ssrf_guard.rs
- PlanService
- Classes
- web_fetch.rs
- 🚀 **Core Coding Features**
- JSON-to-Database Plan Storage Migration
- agent/memory.rs
- MockProvider
- agent.rs
- .execute
- ProviderUpdater
- Database
- MessageRepository
- notebook.rs
- .execute
- 02-architecture.md
- http.rs
- AskUserTool
- WebSearchTool
- Testing Crustly with Ornith-1.0 9B (`ornith:9b`)
- 09-json-output.md
- 03-mixture-of-experts.md
- file_read_cache.rs
- String
- CodeExecTool
- glob.rs
- Enhanced Tool System
- LsTool
- edit.rs
- functions/index.md
- Approval Callback (async callback pattern)
- Clipboard Copy/Paste Integration (arboard)
- Local LLM & Catwalk Analysis
- OllamaProviderConfig
- PlanDocument
- 🦙 Using Crustly with Ollama
- app/mod.rs
- CrustlyError
- GeminiProvider
- validate_file_path
- plan_crash_recovery_test.rs
- parallel_tool_dispatch.rs
- Git Commit Summary
- 08-tool-calling.md
- Proposed Ollama Provider
- Startup Splash Screen
- Crustly Main Chat Interface
- logo.rs
- factory.md
- SqlitePool
- qwen.rs
- Modules
- dialogs/mod.rs
- PromptAnalyzer
- CLI Framework (Clap v4)
- Changelog
- Database Layer (SQLite + SQLx)
- gemini.rs
- extract_think_tags
- Database Lock Recovery
- Completed Milestones
- Mock Providers (Streaming/Error/Working)
- 01-introduction.md
- AzureOpenAIProvider
- MCPClient.md
- LM Studio
- crustly.md
- check_plan_debug.sh
- Croissant Brand Image
- .handle_event
- test_ctrl_keys.sh
- test_plan_mode.sh
- gemma-4-26b-a4b/README.md
- plan_tests.rs
- 📝 Local Configuration with crustly.toml
- .to_qwen_request
- gemini.md
- 💡 Best Practices for Using Crustly
- registry.md
- 07-thinking-mode.md
- Architecture Documentation
- 📈 Status
- services/message.md
- ProviderError
- decisions/README.md
- String
- Dependencies (by category, from Cargo.toml)
- Currently Implemented (Sprint 11 Complete ✅)
- services/session.md
- .stream
- 20-benchmarks.md
- CI Workflow
- database.rs
- aliases.rs
- Supported Tasks
- openai.md
- powershell.md
- qwen.md
- plan_tests.md
- 9. Capacités principales
- Session
- bash.md
- skill.md
- handle_chat_key.md
- types.md
- send_message_with_tools_inner.md
- next.md
- src/plan.md
- run_migrations.md
- plan_tool_security_tests.md
- switch_mode.md
- tui/events.md
- tui/app.md
- plan_mode_integration_test.rs
- error_scenarios_test.md
- task.md
- classes/index.md
- client.md
- agent/memory.md
- render_to_string.md
- edit.md
- cli.md
- todo_write.md
- TaskStatus
- tui/error.md
- models.md
- ollama_models.md
- integration_test.md
- runner.md
- TuiEvent
- cache.md
- render/render.md
- tools/trait.md
- plan_autorun_test.rs
- Architecture Validation
- apply_patch.md
- sandbox.md
- NNNN. Short title of the decision
- Module Coupling: cargo-modules vs graphify communities
- save_memory.md
- src/config.md
- db/retry.md
- web_fetch.md
- handle_event.md
- handle_key_event.md
- azure.md
- handle_model_download_key.md
- create_new_session.md
- repository/session.md
- anthropic.md
- ollama.md
- key.md
- notebook.md
- DummyProvider
- Architecture Drift Check
- plan_mode_integration_test.md
- provider/trait.md
- tools/agent.md
- tools/context.md
- handle_plan_key.md
- set_input_text.md
- highlight.md
- llama_cpp.rs
- router.md
- write.md
- pdf_context.md
- PlanTool/tool/execute.md
- render_help.md
- services/plan.md
- SessionService
- with_context.md
- to_gemini_request.md
- services.md
- ollama_download.md
- plan_tool.md
- AGENTS.md
- CLAUDE.md
- Containers (C4 L2)
- read.md
- logging.md
- tool_call_from_content.md
- tui/render.md
- file_read_cache.md
- glob.md
- grep.md
- http.md
- from_ollama_response.md
- llama_cpp_models.rs
- tool_call_recovery.rs
- ask_user.md
- update-knowledge-graph.sh
- doc_parser.md
- architecture/context.md
- ls.md
- repository/message.md
- parse_fallback_tool_calls.md
- compact.md
- post-checkout
- post-commit
- analyze-module-coupling.sh
- check-architecture-drift.sh
- generate-architecture-docs.sh
- generate-ctags.sh
- setup-graphify-hooks.sh
- validate-architecture.sh
- web_search.md
- Contributing Guidelines
- AgentService
- Provider Trait
- Read-Only Mode
- Repository Pattern
- Streaming Architecture
- SubAgentLauncher Trait
- Reasoning/Thinking Display Pipeline
- Tool Trait
- ToolRegistry
- TUI App
- Catwalk Provider Registry Service
- Catwalk Client Library
- Catwalk Integration (Spec)
- Context File Loading
- Profiling Support (pprof)
- Secret Management (zeroize)
- Shell Detection System
- Crustly Final Specification v3.0
- Modal Approval Dialog
- Auto-Approve Mode
- Capability-Based Permissions
- Channel-Based Communication
- Tool Approval System
- Agent Service Integration (approval)
- Approval System Implementation Notes
- Code Quality Report (Sprint 0)
- Windows MSVC dlltool Build Issue
- Approval System v2 Enhancements
- Error Recovery & Resilience
- Local LLM Integration
- LSP Integration
- MCP Protocol Support
- Multi-Provider Support
- Plugin System (WASM)
- Quick Wins (High Value, Low Effort)
- Security Hardening
- Claude Code Plan Mode (external reference)
- PlanDocument Structure
- Plan Mode
- PlanTask Structure
- Plan Mode Tool Restrictions
- Priority Matrix
- Project Rename: crusty to crustly
- Crabrace Integration (provider registry)
- Error Handling System (CrustlyError + ErrorCode)
- Sprint 0: Project Setup and Initialization
- Approval Timeout (auto-deny)
- Database Benchmark Suite (criterion)
- OpenAI Provider
- Sprint 10: Quick Wins + Multi-Provider Start
- Error Reporting Infrastructure (ErrorInfo)
- Rate Limit Detection & Handling
- Retry Logic with Exponential Backoff
- Sprint 11: Error Recovery & Resilience
- Repository Pattern
- Sprint 1: Database Layer
- Token & Cost Tracking
- Model Type Alignment
- Service Layer (Session/Message/File services)
- Sprint 3: Service Layer
- Agent Service
- Provider Abstraction Layer
- Sprint 4 Plan: LLM Integration
- Tool Framework (Tool trait + ToolRegistry)
- Sprint 5 Plan: Terminal User Interface
- Terminal User Interface (Ratatui)
- Event-Driven TUI Architecture
- Sprint 5 Progress: TUI Phase 1
- Component Wiring (CLI to TUI)
- Sprint 6: Runnable Application
- Sprint 8: Enhanced Testing
- Sprint 8 Plan: Enhanced Testing
- Markdown Rendering (pulldown-cmark)
- Sprint 9: Enhanced TUI Experience
- Syntax Highlighting (syntect)
- Sprint 9 Plan: Enhanced TUI Experience
- Testing Summary
- MinGW-w64 Toolchain
- Windows dlltool.exe Build Issue
- Sprint 0 (Project Initialization)
- Catwalk (Superseded Provider Registry)
- Crabrace Provider Registry Server
- CrabraceConfig
- CrabraceIntegration
- LM Studio Local Provider (Crabrace Registry)
- Ollama Local Provider
- Automatic Provider Discovery
- Conditional File Logging
- init_logging
- LogConfig
- Log Management CLI (crustly logs)
- Rust tracing Ecosystem
- LM Studio Local LLM Server
- OPENAI_BASE_URL Environment Variable
- OpenAI-Compatible API
- Cost and Token Tracking (Test Coverage)
- Crustly Manual Test Scenarios
- Session Management (Test Coverage)
- Crustly TUI
- Regex Keyword Detection
- Prompt Analyzer
- send_message (TUI App Integration)
- Hermes-Style Tool Calling
- Native Qwen-Agent Tool Format
- QwenProvider
- vLLM Inference Server
- Non-Interactive Run Command
- Implementation Summary
- PLAN.md Export
- Plan Mode
- Specification Review (Crustly vs Crush)
- sandbox.rs
- Circular Dependency Detection
- complete_response
- execute_next_plan_task
- execute_plan_tasks
- Plan Mode Read-Only Tool Restriction
- Topological Sort of Task Dependencies
- Kitty Keyboard Protocol Handling
- Model Info Panel (Ctrl+O)
- Provider Switch Dialog (Ctrl+W)
- Slash-Command Interception Layer
- tui-textarea Input Migration
- FSL-1.1-MIT License
- PR #14 — ollama-rs Integration
- PR #15 — TUI Ergonomics
- Ollama Embeddings Capability
- ollama-rs Integration Plan
- Model Download Dialog (Ctrl+D)
- ollama-rs Crate
- OllamaProvider (Native)
- PerfMetrics
- Auto Mode
- Crush (Go Original)
- Local LLM Support
- Native Ollama Provider
- Ollama
- OS Keyring API Key Storage
- Phase 4 Tools (Claw Code Parity)
- /skills and /mcp Discovery Commands
- finish.md
- repository/file.md
- repository/memory.md
- record.md
- render_approval.md
- from_qwen_response.md
- repository/plan.md
- from_config.md
- add_message.md
- to_qwen_request.md
- with_tool_parser.md
- setup_test_service.md
- db.md
- provider/retry.md
- ssrf_guard.md
- cmd_chat.md
- cli/run.md
- plan_crash_recovery_test.md
- default_local.md
- test_to_ollama_request_maps_common_fields.md
- dashscope_intl.md
- compaction_test.md
- interfaces/index.md
- registry.rs
- code_exec.md
- OllamaProvider/provider/stream.md
- MarkdownRenderer
- parse_patch.md
- event_sender.md
- parse_markdown.md
- repository/plan.rs
- ApplyPatchTool/tool/execute.md
- LlamaCppProvider
- parse_markdown
- config/update.md
- markdown.md
- cmd_ollama.md
- HttpClientTool/tool/execute.md
- ollama_download.rs
- agent/context.md
- parse.md
- check_path.md
- `llama-cpp-2` Integration Plan
- Spécification d'évaluation : chargement direct de modèles GGUF via `llama-cpp-2`
- Stratégie de différenciation : Crustly face à OpenCode
- ⚙️ Using Crustly with llama.cpp (in-process, no server)
- crabrace.md
- Config/load.md
- ollama_provider_from_config.md
- retry_with_backoff.md
- aliases.md
- context_with_file.md
- make_root.md
- logo.md
- parse_plain_text.md
- render_chat.md
- llama_cpp_grammar.rs
- src/app.md
- parallel_tool_dispatch.md
- apply_env_overrides.md
- interrupted_plan_from_tasks.md
- apply_hunks.md
- list_files_for_session.md
- ServiceContext
- pdf_context.rs
- Crustly Performance Improvement Plan
- utils/retry.md
- plan_from_db.md
- from_gemini_response.md
- is_success.md
- to_ollama_request.md
- extract_thinking.md
- parse_hermes_tool_calls.md
- flush_current_line.md
- render_message_lines.md
- render_model_download.md
- agent/compaction.rs
- Config/default/default.md
- ToolCacheConfig/default/default.md
- init_minimal_logging.md
- get_most_recent_plan.md
- history_prev.md
- dialogs.md
- handle_end_tag.md
- render_plan.md
- provider/error.md
- services/file.md
- estimate_tokens.md
- is_blocked_ip.md
- 4. Architecture
- run_stream
- parse_patch
- Interfaces
- cmd_logs.md
- plan_to_db.md
- with_think.md
- DocParserTool/tool/execute.md
- normalize_path.md
- import_from_json.md
- 13. Phasing
- DummyProvider
- CrabraceConfig/default/default.md
- update_task_status.md
- AnthropicProvider/provider/complete.md
- is_local.md
- parse_docx.md
- test_task_failure.md
- get_file.md
- classify_tier.md
- render_splash.md
- model_routing_test.md
- plan_autorun_test.md
- v0.5.2 — Local-Model Reliability & Per-Model Tuning
- 🔒 Crustly's Security & Permission Model
- prompt_analyzer.md
- merge_from_file.md
- get_file_required.md
- String
- ROADMAP.md
- src/error.md
- to_policy.md
- find_file_by_path.md
- task_to_db.md
- task_from_db.md
- trim_to_fit.md
- token_count.md
- parse_anthropic_sse_stream.md
- list_models.md
- stream_events_from_buffered_content.md
- extract_retry_after.md
- parse_html.md
- search_file.md
- skip.md
- setup_from_cli.md
- call_tool.md
- progress_percentage.md
- copy_last_response_to_clipboard.md
- switch_provider_to_ollama_model.md
- analyze_and_transform.md
- retry/retry.md
- src/utils.md
- ApplyPatchTool
- normalize_path
- compaction_test.rs
- MemoryConfig/default/default.md
- get_task_mut.md
- is_vision_model.md
- map_ollama_error.md
- with_num_ctx.md
- to_ollama_tool.md
- mock_sse_server.md
- build_client.md
- approval_dialog_details_view_shows_pretty_printed_json.md
- test_preset_configs.md
- 4. Étude complète des bénéfices
- build_approval_callback
- repository/compaction.md
- model_hints.md
- mcp_contract_test.md
- get_provider.md
- ProviderUpdater/update.md
- row_to_plan_task.md
- strip_html_tags.md
- get_or_create_file.md
- is_file_tracked.md
- syntect_style_to_ratatui.md
- .extract_thinking
- agent/error.md
- fetch_providers.md
- health_check.md
- default_ollama_host.md
- should_update.md
- test_update_result_failure.md
- format_hermes_tools.md
- format_native_qwen_result.md
- format_native_qwen_tools.md
- with_thinking_budget.md
- OrPolicy/permissionpolicy/evaluate.md
- reject.md
- dependencies_satisfied.md
- can_retry.md
- advance.md
- count_files_in_session.md
- test_delete_files_for_session.md
- supported_languages.md
- build_keyword_regex.md
- MockProvider
- TestError
- create_pool_with_schema
- benchmark-vs-opencode.sh
- render_splash
- parse_xml.md
- handle_rule.md
- Benchmarks
- test_config_save_and_load.md
- test_parse_markdown_file.md
- test_unsupported_format.md
- file/create_test_service.md
- test_service_update.md
- test_parse_code_block.md
- test_multiple_detections.md
- test_plan_detection.md
- test_read_file_detection.md
- test_search_detection.md
- test_web_search_detection.md
- header_omits_tokens_per_second_when_unavailable.md
- mcp_view_shows_connection_error.md
- model_download_confirm_delete_shows_prompt.md
- retry_after.md
- test_non_retryable_error_fails_immediately.md
- test_retryable_error_retries.md
- test_successful_operation_no_retry.md
- copilot-instructions.md
- Config.md
- DatabaseConfig.md
- McpConfig.md
- OllamaModelConfig.md
- OllamaProviderConfig.md
- SecurityConfig.md
- ThinkSetting.md
- ToolCacheConfig.md
- AnthropicResponse.md
- AllowAll.md
- AllowToolRule.md
- AndPolicy.md
- BashCommandAllowlist.md
- NotPolicy.md
- TestError.md
- default_risk_threshold.md
- config/default_true.md
- model_override_respects_provider_precedence.md
- model_override_skips_gemini_without_api_key.md
- QwenProviderConfig/default/default.md
- test_config_validation.md
- test_config_validation_empty_crabrace_url.md
- test_database_config_default.md
- test_logging_config_default.md
- test_provider_configs_default.md
- ttl_secs_for.md
- PlanTaskRepository/new.md
- test_plan_create.md
- test_plan_risks_serialization.md
- test_plan_status_conversion.md
- test_plan_tasks_cascade_delete.md
- test_plan_with_no_tasks.md
- test_task_dependencies_serialization.md
- test_task_status_conversion.md
- AnthropicProvider/handle_error.md
- AnthropicProvider/new.md
- AnthropicProvider/provider/calculate_cost.md
- absolute_path_to_nonexistent_file_outside_root_still_denied.md
- AllowToolRule/new.md
- and_policy_denies_trusted_command_that_a_later_rule_rejects.md
- bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval.md
- bash_allowlist_never_trusts_shell_operator_chaining.md
- deny_path_prefix_blocks_matching_path.md
- DenyPathPrefixRule/new.md
- not_policy_inverts_allow.md
- not_policy_inverts_trusted_to_deny.md
- or_policy_short_circuits_on_allow.md
- evaluate-2.md
- PathBoundaryRule/permissionpolicy/evaluate.md
- api_aggressive.md
- utils/retry/RetryConfig/calculate_delay.md
- utils/retry/RetryConfig/no_retry.md
- test_calculate_delay_capped.md
- retryableerror/is_retryable.md

## God Nodes (most connected - your core abstractions)
1. `Functions` - 2265 edges
2. `Classes` - 323 edges
3. `Modules` - 126 edges
4. `App` - 112 edges
5. `test_app()` - 88 edges
6. `LLMRequest` - 77 edges
7. `PlanDocument` - 55 edges
8. `ToolExecutionContext` - 53 edges
9. `QwenProvider` - 51 edges
10. `Provider` - 50 edges

## Surprising Connections (you probably didn't know these)
- `setup_test_db()` --references--> `Database`  [EXTRACTED]
  benches/database.rs → src/db/mod.rs
- `create_error_agent()` --references--> `Database`  [EXTRACTED]
  tests/error_scenarios_test.rs → src/db/mod.rs
- `create_test_db()` --references--> `Database`  [EXTRACTED]
  tests/error_scenarios_test.rs → src/db/mod.rs
- `create_test_agent()` --references--> `Database`  [EXTRACTED]
  tests/integration_test.rs → src/db/mod.rs
- `create_test_db()` --references--> `Database`  [EXTRACTED]
  tests/integration_test.rs → src/db/mod.rs

## Import Cycles
- 2-file cycle: `src/services/file.rs -> src/services/mod.rs -> src/services/file.rs`
- 2-file cycle: `src/services/mod.rs -> src/services/session.rs -> src/services/mod.rs`
- 2-file cycle: `src/services/mod.rs -> src/services/plan.rs -> src/services/mod.rs`
- 2-file cycle: `src/services/message.rs -> src/services/mod.rs -> src/services/message.rs`
- 3-file cycle: `src/services/file.rs -> src/services/session.rs -> src/services/mod.rs -> src/services/file.rs`
- 3-file cycle: `src/services/message.rs -> src/services/session.rs -> src/services/mod.rs -> src/services/message.rs`

## Hyperedges (group relationships)
- **Crustly TUI Screenshot Gallery** — docs_screenshots_main_screen_chat_interface, docs_screenshots_deeply_analyse_deep_code_analysis, docs_screenshots_thinking_mode_thinking_indicator, docs_screenshots_help_screnn_help_screen, docs_screenshots_screenshot_2025_11_23_002028_lm_studio_session [INFERRED 0.90]

## Communities (680 total, 248 thin omitted)

### Community 0 - "FileService"
Cohesion: 0.19
Nodes (21): create_test_service(), FileService, Option, Path, PathBuf, Result, Self, String (+13 more)

### Community 1 - "SessionRepository"
Cohesion: 0.22
Nodes (9): Option, Result, Self, Uuid, Vec, SessionListOptions, SessionRepository, test_session_archive() (+1 more)

### Community 2 - "service.rs"
Cohesion: 0.06
Nodes (51): AgentResponse, AgentService, AgentServiceLauncher, AgentStreamResponse, apply_streamed_tool_input(), create_test_service(), drain_stream_assembles_anthropic_tool_input_from_json_deltas(), drain_stream_to_response() (+43 more)

### Community 3 - "events.rs"
Cohesion: 0.09
Nodes (40): is_approve(), is_cancel(), is_clear_session(), is_copy_response(), is_deny(), is_down(), is_enter(), is_help() (+32 more)

### Community 4 - "openai.rs"
Cohesion: 0.08
Nodes (42): configure_openai(), AuthStyle, default_auth_style_still_sends_bearer(), OpenAIChoice, OpenAIError, OpenAIErrorResponse, OpenAIFunction, OpenAIFunctionCall (+34 more)

### Community 5 - "ollama.rs"
Cohesion: 0.06
Nodes (66): ChatMessage, ChatMessageFinalResponseData, ChatMessageRequest, ChatMessageResponse, FormatType, KeepAlive, OllamaError, bash_tool() (+58 more)

### Community 6 - "Provider"
Cohesion: 0.07
Nodes (45): AtomicUsize, absent_llama_cpp_config_does_not_affect_resolution(), configure_qwen(), configure_qwen_auto_selects_openai_parser_for_coder_next(), configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection(), configure_qwen_keeps_hermes_default_for_other_models(), create_anthropic(), create_provider() (+37 more)

### Community 7 - "render.rs"
Cohesion: 0.05
Nodes (110): B, Signature, Signature, approval_dialog_area(), approval_dialog_details_view_shows_pretty_printed_json(), approval_dialog_shows_tool_name_capabilities_and_summarized_params(), chat_input_renders_textarea_contents_and_hint(), chat_message_perf_footer_reports_cold_and_warm_starts() (+102 more)

### Community 8 - "AgentContext"
Cohesion: 0.12
Nodes (22): DbMessage, AgentContext, Message, Option, PathBuf, Result, Self, String (+14 more)

### Community 9 - "File"
Cohesion: 0.26
Nodes (10): File, FileRepository, Option, Path, Result, Self, Uuid, Vec (+2 more)

### Community 10 - "app.rs"
Cohesion: 0.08
Nodes (68): altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut(), altgr_backslash_reaches_the_input(), auto_mode_defaults_to_interactive(), chat_alt_enter_inserts_newline_as_non_kitty_fallback(), chat_backspace_deletes_at_cursor_not_always_the_last_char(), chat_ctrl_backspace_deletes_whole_word(), chat_ctrl_enter_still_submits_as_legacy_alias(), chat_ctrl_left_right_jump_by_word() (+60 more)

### Community 11 - "PermissionPolicy"
Cohesion: 0.11
Nodes (20): Pattern, AllowToolRule, and_policy_denies_trusted_command_that_a_later_rule_rejects(), and_policy_does_not_trust_unlisted_program(), and_policy_preserves_trust_when_no_rule_denies(), AndPolicy, BashCommandAllowlist, DenyToolRule (+12 more)

### Community 12 - "PlanRepository"
Cohesion: 0.17
Nodes (12): PlanRepository, PlanTaskRepository, row_to_plan_task(), Option, PlanTask, Result, Self, String (+4 more)

### Community 13 - "SecretString"
Cohesion: 0.05
Nodes (44): D, Deserialize, Ok, S, Serialize, ProviderSecrets, Debug, Default (+36 more)

### Community 14 - "ModelRouter"
Cohesion: 0.07
Nodes (23): Signature, all_tiers_resolve_to_non_empty_model(), ModelRouter, ModelTier, Default, Option, Self, String (+15 more)

### Community 15 - ".new"
Cohesion: 0.10
Nodes (14): DisplayMessage, llama_cpp_delete_finished_removes_model_from_list(), llama_cpp_switch_finished_swaps_provider_in_place(), quantization_hint_for_path(), Arc, DateTime, From, Message (+6 more)

### Community 16 - "plan_tool_security_tests.rs"
Cohesion: 0.07
Nodes (32): acceptance_criteria_are_surfaced_at_start_and_completion(), completing_without_criteria_warns(), default_task_type(), PlanOperation, PlanTool, Option, Path, Result (+24 more)

### Community 17 - "highlight.rs"
Cohesion: 0.13
Nodes (22): find_syntax(), get_theme(), highlight_code(), is_language_supported(), Color, Line, Option, String (+14 more)

### Community 18 - "task.rs"
Cohesion: 0.10
Nodes (26): Drop, FileLock, get_store_path(), parse_priority(), parse_status(), DateTime, F, HashMap (+18 more)

### Community 19 - "skill.rs"
Cohesion: 0.10
Nodes (29): frontmatter_name_matches(), list_skills(), list_skills_deduplicates_same_name_across_roots(), list_skills_discovers_legacy_flat_md_files(), list_skills_discovers_project_local_skills_with_frontmatter(), list_skills_does_not_panic_on_a_directory_with_no_skills_dir(), list_skills_falls_back_to_directory_name_without_frontmatter_name(), list_skills_is_sorted_alphabetically_case_insensitive() (+21 more)

### Community 20 - "doc_parser.rs"
Cohesion: 0.12
Nodes (26): context_with_file(), DocParserInput, DocParserTool, DocumentMetadata, ParsedMetadata, Option, Path, PathBuf (+18 more)

### Community 21 - "todo_write.rs"
Cohesion: 0.10
Nodes (26): default_priority(), ReadInput, render_todos(), DateTime, Display, Formatter, Path, Result (+18 more)

### Community 22 - "powershell.rs"
Cohesion: 0.07
Nodes (13): execute_allows_read_only_command_in_plan_mode(), execute_blocks_dangerous_command_in_read_only_mode(), is_read_only_powershell(), make_ctx(), PowerShellInput, PowerShellTool, Option, Result (+5 more)

### Community 23 - "ToolRegistry"
Cohesion: 0.14
Nodes (13): MockTool, preview_input(), preview_input_truncates_a_large_payload(), preview_input_truncates_on_char_boundaries(), Default, HashMap, Option, Result (+5 more)

### Community 24 - "utils/retry.rs"
Cohesion: 0.15
Nodes (19): C, retry(), retry_with_check(), RetryConfig, Default, Duration, E, F (+11 more)

### Community 25 - "logging.rs"
Cohesion: 0.16
Nodes (24): Level, cleanup_old_logs(), debug_filter_is_scoped_to_crustly(), debug_log_files_are_findable_by_the_readers(), get_log_path(), init_debug_logging(), init_logging(), init_minimal_logging() (+16 more)

### Community 26 - "client.rs"
Cohesion: 0.09
Nodes (26): BufReader, Child, ChildStdin, ChildStdout, JsonRpcRequest, JsonRpcResponse, match_response_line(), mcp_tool_always_requires_approval_regardless_of_empty_capabilities() (+18 more)

### Community 27 - "Config"
Cohesion: 0.11
Nodes (35): build_tool_registry(), build_tool_registry_registers_every_built_in_tool(), Cli, cmd_autoplan(), cmd_chat(), cmd_config(), cmd_db(), cmd_init() (+27 more)

### Community 28 - "ErrorInfo"
Cohesion: 0.14
Nodes (14): ErrorCategory, ErrorInfo, ErrorSeverity, Color, DateTime, From, Option, Self (+6 more)

### Community 29 - "apply_patch.rs"
Cohesion: 0.21
Nodes (19): apply_hunks(), apply_hunks_errors_when_context_not_found(), apply_hunks_replaces_matched_context(), apply_hunks_second_hunk_searches_after_first(), context(), execute_add_and_delete_need_no_prior_read(), execute_add_file_that_already_exists_fails(), execute_adds_a_new_file() (+11 more)

### Community 30 - "error_scenarios_test.rs"
Cohesion: 0.12
Nodes (21): create_error_agent(), create_test_db(), ErrorMockProvider, ErrorType, Option, ProviderResult, ProviderStream, Result (+13 more)

### Community 31 - "ollama_models.rs"
Cohesion: 0.14
Nodes (22): client_for(), delete_model(), delete_model_succeeds_on_2xx(), generate_embeddings(), generate_embeddings_parses_response(), invalid_host_returns_error(), list_models(), list_models_parses_tags_response() (+14 more)

### Community 32 - "QwenProvider"
Cohesion: 0.14
Nodes (7): QwenProvider, Client, Self, test_custom_default_model(), test_sampling_config_override_wins_over_defaults(), test_thinking_mode_configuration(), ToolCallParser

### Community 33 - "config/mod.rs"
Cohesion: 0.07
Nodes (36): allow_bash_trusts_only_the_listed_read_only_programs(), default_compaction_threshold(), default_db_path(), default_episodic_budget(), default_glob_ttl(), default_grep_ttl(), default_log_level(), default_read_file_ttl() (+28 more)

### Community 34 - "Functions"
Cohesion: 0.00
Nodes (2265): absolute_path_outside_root_denied, absolute_path_to_nonexistent_file_in_subdir_allowed, absolute_path_to_nonexistent_file_inside_root_allowed, absolute_path_to_nonexistent_file_outside_root_still_denied, absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed, acceptance_criteria_are_surfaced_at_start_and_completion, acquire, add_artifact (+2257 more)

### Community 35 - "MockProvider"
Cohesion: 0.17
Nodes (19): create_test_agent(), create_test_db(), MockProvider, Mutex, Option, ProviderStream, Result, Self (+11 more)

### Community 36 - "Self"
Cohesion: 0.22
Nodes (5): P, Path, Result, Self, test_local_config_path()

### Community 37 - "ContextStore"
Cohesion: 0.14
Nodes (18): ContextEntry, ContextInput, ContextOperation, ContextStore, ContextTool, get_store_path(), DateTime, HashMap (+10 more)

### Community 38 - "StreamingMockProvider"
Cohesion: 0.15
Nodes (18): Option, ProviderResult, ProviderStream, Result, Self, String, Vec, StreamingMockProvider (+10 more)

### Community 39 - "AnthropicProvider"
Cohesion: 0.08
Nodes (32): Bytes, Item, AnthropicError, AnthropicErrorDetail, AnthropicProvider, AnthropicRequest, AnthropicResponse, AnthropicTokenUsage (+24 more)

### Community 40 - "LLMRequest"
Cohesion: 0.08
Nodes (36): ContentBlock, ContentDelta, extract_think_tags(), extract_think_tags_multiple_blocks(), extract_think_tags_no_tags(), extract_think_tags_only_block(), extract_think_tags_single_block(), extract_think_tags_unclosed() (+28 more)

### Community 41 - ".execute"
Cohesion: 0.13
Nodes (16): collect_searchable_files(), GrepInput, GrepTool, Option, Path, PathBuf, Regex, Result (+8 more)

### Community 42 - "ToolExecutionContext"
Cohesion: 0.16
Nodes (17): Arc, Debug, HashMap, Option, PathBuf, Self, Send, String (+9 more)

### Community 43 - "db/retry.rs"
Cohesion: 0.18
Nodes (20): DbRetryConfig, is_database_locked(), retry_db_anyhow(), retry_db_operation(), retry_db_sqlx(), Default, Duration, E (+12 more)

### Community 44 - "App"
Cohesion: 0.08
Nodes (14): McpServerStatus, Option, String, App, Instant, JoinHandle, KeyEvent, PathBuf (+6 more)

### Community 45 - "CrabraceIntegration"
Cohesion: 0.16
Nodes (16): CrabraceClient, CrabraceConfig, CrabraceIntegration, default_auto_update(), default_base_url(), default_enabled(), default_update_interval(), Default (+8 more)

### Community 46 - "ToolResultCache"
Cohesion: 0.17
Nodes (18): DashMap, Fn, cache_expires_after_ttl(), cache_hit_returns_same_result(), CacheEntry, CacheKey, invalidate_matching_drops_selected_tools_and_keeps_others(), Default (+10 more)

### Community 47 - "MessageService"
Cohesion: 0.19
Nodes (22): create_test_service(), create_then_update_survives_a_file_backed_wal_pool(), MessageService, Message, Option, Result, Self, String (+14 more)

### Community 48 - ".local"
Cohesion: 0.11
Nodes (23): Value, test_clean_incomplete_markers(), test_fallback_does_not_corrupt_unrelated_fenced_code_block(), test_fallback_ignores_unrelated_json(), test_fallback_parses_bare_json_tool_call(), test_fallback_parses_fenced_json_tool_call(), test_fallback_rejects_unregistered_tool_name(), test_find_json_objects_recovers_nested_object_after_failed_outer_parse() (+15 more)

### Community 50 - "retry_with_backoff"
Cohesion: 0.17
Nodes (20): extract_retry_after(), parse_retry_seconds(), retry_with_backoff(), retry_with_rate_limit(), RetryConfig, Default, Duration, F (+12 more)

### Community 51 - "bash.rs"
Cohesion: 0.09
Nodes (21): bash_runs_posix_in_the_requested_working_directory(), BashInput, BashTool, is_read_only_command(), resolve_shell(), Option, Result, String (+13 more)

### Community 52 - "ssrf_guard.rs"
Cohesion: 0.09
Nodes (18): ClientBuilder, IpAddr, Ipv4Addr, Ipv6Addr, Name, Policy, Resolve, Resolving (+10 more)

### Community 53 - "PlanService"
Cohesion: 0.14
Nodes (26): create_test_plan(), PlanService, PlanStatistics, PlanValidationWarning, Option, Path, PlanTask, Result (+18 more)

### Community 54 - "Classes"
Cohesion: 0.01
Nodes (323): AgentContext, AgentError, AgentInput, AgentManifest, AgentResponse, AgentService, AgentServiceLauncher, AgentStreamResponse (+315 more)

### Community 55 - "web_fetch.rs"
Cohesion: 0.12
Nodes (15): execute_denies_cloud_metadata_endpoint(), execute_denies_loopback_address(), html_to_text(), Result, String, Tool, Value, Vec (+7 more)

### Community 56 - "🚀 **Core Coding Features**"
Cohesion: 0.08
Nodes (24): 1. **Built-in Tool Execution System**, 2. **Syntax Highlighting for 100+ Languages**, 3. **Markdown Code Blocks**, 4. **Multi-line Input with Real Cursor Editing**, 5. **Session-Based Context**, 6. **Terminal-Native Workflow**, 7. **Local LLM Support (Privacy)**, 8. **Streaming Responses** (+16 more)

### Community 57 - "JSON-to-Database Plan Storage Migration"
Cohesion: 0.40
Nodes (5): SQLx Database Library, JSON-to-Database Plan Storage Migration, PlanService, .crustly_plan.json State File, Plan Mode Sequential Task Execution

### Community 58 - "agent/memory.rs"
Cohesion: 0.21
Nodes (16): CodebaseIndex, CodebaseIndexEntry, EpisodicMemory, extract_symbols(), row_to_entry(), DateTime, Path, Result (+8 more)

### Community 59 - "MockProvider"
Cohesion: 0.11
Nodes (11): Pin, MockProvider, ProviderCapabilities, Option, ProviderStream, Result, Self, String (+3 more)

### Community 60 - "agent.rs"
Cohesion: 0.14
Nodes (13): AgentInput, AgentManifest, AgentTool, Option, Result, String, Tool, Value (+5 more)

### Community 61 - ".execute"
Cohesion: 0.13
Nodes (15): ReadInput, ReadTool, Option, Path, Result, String, Tool, Value (+7 more)

### Community 62 - "ProviderUpdater"
Cohesion: 0.18
Nodes (11): ProviderUpdater, Option, Result, Self, String, SystemTime, test_should_update_when_disabled(), test_should_update_when_never_updated() (+3 more)

### Community 63 - "Database"
Cohesion: 0.20
Nodes (11): Database, deleting_a_session_cascades_to_its_messages(), foreign_keys_are_enforced(), migrating_from_pre_modernization_schema_preserves_existing_messages(), PoolExt, P, Result, Self (+3 more)

### Community 64 - "MessageRepository"
Cohesion: 0.27
Nodes (9): MessageRepository, Message, Option, Result, Self, Uuid, Vec, test_message_crud() (+1 more)

### Community 65 - "notebook.rs"
Cohesion: 0.15
Nodes (14): Cell, minimal_notebook_json(), Notebook, NotebookEditTool, NotebookInput, NotebookOperation, Option, Result (+6 more)

### Community 66 - ".execute"
Cohesion: 0.13
Nodes (17): Result, String, Tool, Value, Vec, test_creating_a_new_file_needs_no_prior_read(), test_overwrite_existing_file(), test_overwrite_rejects_a_file_changed_since_it_was_read() (+9 more)

### Community 67 - "02-architecture.md"
Cohesion: 0.08
Nodes (24): 10. Le Router, 11. Experts spécialisés, 12. Paramètres actifs, 13. Impact sur Ollama, 14. KV Cache, 15. Impact du contexte sur la mémoire, 16. Architecture et RTX 3060, 17. Recommandation Crustly (+16 more)

### Community 68 - "http.rs"
Cohesion: 0.13
Nodes (13): Method, execute_denies_cloud_metadata_endpoint(), execute_denies_loopback_address(), HttpClientTool, HttpInput, parse_method(), HashMap, Option (+5 more)

### Community 69 - "AskUserTool"
Cohesion: 0.14
Nodes (12): AskUserInput, AskUserTool, Option, Result, String, Tool, Value, Vec (+4 more)

### Community 70 - "WebSearchTool"
Cohesion: 0.17
Nodes (10): DuckDuckGoResponse, RelatedTopic, Result, String, Tool, Value, Vec, SearchInput (+2 more)

### Community 71 - "Testing Crustly with Ornith-1.0 9B (`ornith:9b`)"
Cohesion: 0.08
Nodes (23): 10. Sign-off checklist, 11. References, 1. The model under test, 2. Required hardware, 3.1 Software prerequisites, 3.2 Build matrix (regression gate — run before any manual step), 3.3 Crustly config, 3. Environment setup (+15 more)

### Community 72 - "09-json-output.md"
Cohesion: 0.07
Nodes (26): 10. Sortie JSON pour Tool Calling, 11. Validation côté application, 12. Gestion des erreurs JSON, 13. Prompt système recommandé, 14. Paramètres Gemma recommandés, 15. Actions agent recommandées, 16. Exemple workflow complet, 17. JSON Streaming (+18 more)

### Community 73 - "03-mixture-of-experts.md"
Cohesion: 0.07
Nodes (27): 10. MoE et raisonnement, 11. Différence Dense vs MoE, 12. Impact sur la VRAM, 13. Quantification MoE, 14. Impact sur Ollama, 15. Optimisation Crustly, 16. Problèmes possibles du MoE, 17. MoE et agents autonomes (+19 more)

### Community 74 - "file_read_cache.rs"
Cohesion: 0.19
Nodes (17): Metadata, distinct_paths_are_tracked_independently(), FileFingerprint, FileReadCache, fp(), matching_fingerprint_after_record_is_ok(), mismatched_fingerprint_is_stale(), never_read_path_is_rejected() (+9 more)

### Community 75 - "String"
Cohesion: 0.20
Nodes (19): QwenChoice, QwenFunction, QwenFunctionCall, QwenFunctionCallDelta, QwenMessage, QwenMessageDelta, QwenRequest, QwenResponse (+11 more)

### Community 76 - "CodeExecTool"
Cohesion: 0.17
Nodes (7): CodeExecInput, CodeExecTool, Result, String, Tool, Value, Vec

### Community 77 - "glob.rs"
Cohesion: 0.15
Nodes (14): context(), GlobInput, GlobTool, Option, Result, String, TempDir, Tool (+6 more)

### Community 79 - "LsTool"
Cohesion: 0.13
Nodes (14): Future, Output, LsInput, LsTool, Box, Option, Path, PathBuf (+6 more)

### Community 80 - "edit.rs"
Cohesion: 0.12
Nodes (22): context(), EditInput, EditOperation, EditTool, normalize_input(), Result, String, TempDir (+14 more)

### Community 81 - "functions/index.md"
Cohesion: 0.02
Nodes (103): Functions, Called by, Signature, Called by, Calls, Signature, Called by, Calls (+95 more)

### Community 85 - "OllamaProviderConfig"
Cohesion: 0.14
Nodes (18): DatabaseConfig, default_enabled(), default_llama_cpp_n_ctx(), default_ollama_host(), LlamaCppProviderConfig, LoggingConfig, OllamaModelConfig, OllamaProviderConfig (+10 more)

### Community 86 - "PlanDocument"
Cohesion: 0.08
Nodes (20): AutoRunMode, ExecutionSummary, InterruptedPlan, PauseReason, PlanDocument, PlanModeState, PlanTask, DateTime (+12 more)

### Community 87 - "🦙 Using Crustly with Ollama"
Cohesion: 0.11
Nodes (19): "Connection refused" at `localhost:11434`, Increasing context window, "Model not found" error, Option A: Configuration File (Recommended), Option B: Environment Variables (Quick Start), Prerequisites, Quick Troubleshooting Checklist, Recommended Models for Crustly + Ollama (+11 more)

### Community 88 - "app/mod.rs"
Cohesion: 0.26
Nodes (8): App, is_rust_file_in_root(), Default, Path, PathBuf, Result, Self, start_file_watcher()

### Community 89 - "CrustlyError"
Cohesion: 0.39
Nodes (5): CrustlyError, ErrorCode, Error, Option, String

### Community 90 - "GeminiProvider"
Cohesion: 0.15
Nodes (6): GeminiProvider, Client, HeaderMap, ProviderStream, Response, Result

### Community 91 - "validate_file_path"
Cohesion: 0.38
Nodes (9): Error, Path, PathBuf, Result, String, ToolError, validate_directory_path(), validate_file_path() (+1 more)

### Community 92 - "plan_crash_recovery_test.rs"
Cohesion: 0.40
Nodes (8): crash_recovery_resumes_at_correct_task(), create_plan(), create_session(), failed_task_stores_error_without_completion_timestamp(), minimal_task(), PlanTask, Uuid, task_state_transitions_correct_order()

### Community 93 - "parallel_tool_dispatch.rs"
Cohesion: 0.42
Nodes (9): bench_parallel_dispatch(), make_temp_files(), read_parallel(), read_sequential(), Criterion, PathBuf, String, TempDir (+1 more)

### Community 94 - "Git Commit Summary"
Cohesion: 0.04
Nodes (46): Attribution, Branch, By Category, By Sprint, Claude-Specific .gitignore Additions, Clean Working Directory ✅, Code Quality, Code Quality (+38 more)

### Community 95 - "08-tool-calling.md"
Cohesion: 0.07
Nodes (26): 10. Outils recommandés pour Crustly, 11. Exemple workflow développeur, 12. Gestion des erreurs, 13. Bonnes pratiques, 14. Paramètres recommandés Gemma 4, 15. Prompt système recommandé, 16. Sécurité Agent, 17. Résumé (+18 more)

### Community 98 - "Crustly Main Chat Interface"
Cohesion: 0.22
Nodes (9): Deep Code Analysis Session, Qwen 2.5 Coder Local Model, Help & Commands Screen, Session Management Feature, Crustly Main Chat Interface, Keyboard Shortcuts Bar (Ctrl+H/K/L/N/C), Status Bar (Session / Model / Tokens / Cost), Local LLM Session (qwen2.5-coder-14b) (+1 more)

### Community 99 - "logo.rs"
Cohesion: 0.29
Nodes (3): get_logo_with_version(), String, test_logo_with_version()

### Community 100 - "factory.md"
Cohesion: 0.02
Nodes (85): Signature, Signature, Signature, Called by, Signature, Calls, Signature, Called by (+77 more)

### Community 101 - "SqlitePool"
Cohesion: 0.17
Nodes (14): CompactionRecord, SqlitePool, CompactionRecordRepository, Result, Self, Uuid, Vec, create_test_pool() (+6 more)

### Community 102 - "qwen.rs"
Cohesion: 0.16
Nodes (12): QwenError, QwenErrorResponse, stream_events_from_buffered_content(), streaming_assembles_hermes_tool_call_from_buffered_text(), streaming_plain_text_roundtrips_without_tool_calls(), test_calculate_cost_cloud(), test_calculate_cost_local(), test_calculate_cost_unknown_cloud_model_returns_zero() (+4 more)

### Community 103 - "Modules"
Cohesion: 0.02
Nodes (126): agent, agent, aliases, anthropic, app, app, apply_patch, ask_user (+118 more)

### Community 104 - "dialogs/mod.rs"
Cohesion: 0.50
Nodes (7): centered_rect(), render_auto_exec_progress(), render_crash_recovery_dialog(), render_policy_denial(), Frame, PlanTask, Rect

### Community 107 - "Changelog"
Cohesion: 0.10
Nodes (21): `apply_patch` Tool, AWS Bedrock Support, Bug Fixes & Dependency Upgrades, Changelog, Claude Code / Qwen Compatibility Layer, Codebase Index, Context Compaction, Episodic Memory (+13 more)

### Community 109 - "gemini.rs"
Cohesion: 0.11
Nodes (33): build_gemini_error(), gemini_role(), GeminiError, GeminiErrorResponse, parse_gemini_sse(), test_build_gemini_error_api_error_with_body(), test_build_gemini_error_no_body_falls_back_to_unknown(), test_build_gemini_error_rate_limit_no_body() (+25 more)

### Community 112 - "Completed Milestones"
Cohesion: 0.12
Nodes (17): Backlog (Post-1.0), Completed Milestones, Crustly Roadmap, Guiding Principles, Upcoming Milestones, v0.1 — Foundation, v0.2 — Multi-Provider & Advanced Tools, v0.3 — Workflow & Intelligence (+9 more)

### Community 114 - "01-introduction.md"
Cohesion: 0.10
Nodes (19): 10. Fenêtre de contexte 256k, 11. Limites du contexte, 12. Gemma 4 et raisonnement, 13. Pourquoi Gemma 4 est intéressant en local, 14. Cas d'utilisation recommandés, 15. Conclusion, 1. Présentation, 2. Positionnement dans la famille Gemma (+11 more)

### Community 116 - "AzureOpenAIProvider"
Cohesion: 0.15
Nodes (11): AzureOpenAIProvider, Option, ProviderStream, Result, Self, String, Vec, test_azure_context_window() (+3 more)

### Community 119 - "crustly.md"
Cohesion: 0.04
Nodes (35): Signature, Modules, Contains, Imports, Member of, Member of, Imports, Imports (+27 more)

### Community 122 - ".handle_event"
Cohesion: 0.09
Nodes (22): clear_session_is_refused_while_the_current_session_is_processing(), clear_session_proceeds_when_only_another_session_is_processing(), handle_ollama_delete_finished_failure_keeps_installed_and_posts_error(), handle_ollama_delete_finished_success_removes_from_installed_and_posts_message(), handle_ollama_models_listed_updates_installed_list(), handle_ollama_pull_finished_failure_posts_error_message(), handle_ollama_pull_finished_success_posts_chat_message(), handle_ollama_pull_progress_updates_status_and_fraction() (+14 more)

### Community 136 - "gemma-4-26b-a4b/README.md"
Cohesion: 0.12
Nodes (16): Basic API, Crustly Recommendations, Documentation Structure, Gemma 4 26B A4B MoE, Minimum, Model Summary, Next Chapter, Ollama Installation (+8 more)

### Community 137 - "plan_tests.rs"
Cohesion: 0.14
Nodes (24): create_test_plan(), create_test_task(), PlanTask, Uuid, test_add_task(), test_complex_dependency_chain(), test_count_by_status(), test_get_task() (+16 more)

### Community 138 - "📝 Local Configuration with crustly.toml"
Cohesion: 0.13
Nodes (14): Configuration 1: LM Studio (Windows), Configuration 2: Ollama (Linux), Configuration 3: Cloud API (Anthropic), Configuration 4: Multiple Providers (Hybrid), Configuration File Locations, Configuration Tips, Creating Your Local Configuration File, Example Configurations for Different Setups (+6 more)

### Community 139 - ".to_qwen_request"
Cohesion: 0.17
Nodes (8): T, Tool, test_hermes_tools_format(), test_native_qwen_tools_format(), test_sampling_defaults_qwen25_coder_local(), test_sampling_defaults_qwen3_non_thinking(), test_sampling_defaults_unrecognized_model_name_is_conservative(), test_sampling_explicit_request_top_p_wins()

### Community 140 - "gemini.md"
Cohesion: 0.02
Nodes (69): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+61 more)

### Community 141 - "💡 Best Practices for Using Crustly"
Cohesion: 0.14
Nodes (13): 1. **Codebase Exploration**, 2. **Deep Code Analysis**, 3. **Bug Investigation**, 4. **Feature Implementation**, 5. **Documentation Generation**, 6. **Dependency Analysis**, 💡 Best Practices for Using Crustly, ❌ Ineffective Prompts (To Avoid) (+5 more)

### Community 142 - "registry.md"
Cohesion: 0.02
Nodes (69): Signature, Signature, Calls, Signature, Calls, Signature, Signature, Calls (+61 more)

### Community 143 - "07-thinking-mode.md"
Cohesion: 0.15
Nodes (12): 1. Introduction, 2. Activer/désactiver le thinking, 3. Format de sortie, 4. Gestion en conversation multi-tours, 5. Ce qu'Ollama gère automatiquement, 6. Correspondance avec l'implémentation Crustly, 7. Bonnes pratiques, Chapitre suivant (+4 more)

### Community 144 - "Architecture Documentation"
Cohesion: 0.29
Nodes (6): Architecture Documentation, Phase 1 - crate facts and C4 diagrams, Phase 2 - validation (unused deps, import cycles, orphan modules), Phase 3 - module coupling vs graphify communities, Phase 4 - editor symbol index, ADRs, and doc-drift check, What this is not

### Community 145 - "📈 Status"
Cohesion: 0.13
Nodes (15): Next Priorities, Sprint 0-1 Achievements ✅ (Database & Foundation), Sprint 10: Multi-Provider Support + Quick Wins ✅, Sprint 11: Error Recovery & Resilience ✅, Sprint 12: Plan Mode Implementation ✅, Sprint 2 Achievements ✅ (Configuration System), Sprint 3 Achievements ✅ (Service Layer), Sprint 4 Achievements ✅ (LLM Integration) (+7 more)

### Community 146 - "services/message.md"
Cohesion: 0.03
Nodes (75): Called by, Calls, Signature, Calls, Signature, Calls, Signature, Called by (+67 more)

### Community 147 - "ProviderError"
Cohesion: 0.19
Nodes (7): AgentError, String, Uuid, ProviderError, Error, Option, String

### Community 148 - "decisions/README.md"
Cohesion: 0.07
Nodes (24): 0001. Record architecture decisions, Consequences, Context, Decision, 0002. Use sqlx exclusively, not rusqlite, Consequences, Context, Decision (+16 more)

### Community 149 - "String"
Cohesion: 0.18
Nodes (20): GeminiCandidate, GeminiContent, GeminiFunctionCall, GeminiFunctionCallingConfig, GeminiFunctionDeclaration, GeminiFunctionResponse, GeminiGenerationConfig, GeminiInlineData (+12 more)

### Community 150 - "Dependencies (by category, from Cargo.toml)"
Cohesion: 0.08
Nodes (23): Async Runtime, CLI Framework, Concurrent Data Structures, Configuration, Crate, Database, Dependencies (by category, from Cargo.toml), Document Parsing (+15 more)

### Community 151 - "Currently Implemented (Sprint 11 Complete ✅)"
Cohesion: 0.14
Nodes (14): CLI Commands, Configuration System, Cost & Token Tracking, Currently Implemented (Sprint 11 Complete ✅), Database & Persistence, Developer Experience, Error Recovery & Resilience (Sprint 11), ✨ Features (+6 more)

### Community 152 - "services/session.md"
Cohesion: 0.02
Nodes (67): Called by, Signature, Calls, Signature, Called by, Calls, Signature, Called by (+59 more)

### Community 153 - ".stream"
Cohesion: 0.22
Nodes (9): llm_response_to_stream_events(), mock_sse_server(), HeaderMap, ProviderStream, Response, Result, stream_assembles_openai_style_tool_call_across_sse_chunks(), stream_skips_malformed_sse_chunk_and_continues() (+1 more)

### Community 154 - "20-benchmarks.md"
Cohesion: 0.20
Nodes (9): 1. Famille Gemma 4 comparée, 2. Benchmarks généraux, 3. Vision, 4. Audio, 5. Long Context, 6. Lecture des résultats pour Crustly, Benchmarks, Chapitre suivant (+1 more)

### Community 156 - "database.rs"
Cohesion: 0.53
Nodes (8): bench_message_insert(), bench_message_query(), bench_session_create(), bench_session_get(), bench_session_list(), Criterion, TempDir, setup_test_db()

### Community 157 - "aliases.rs"
Cohesion: 0.25
Nodes (3): alias_resolution_is_a_single_hop(), resolve(), Option

### Community 158 - "Supported Tasks"
Cohesion: 0.33
Nodes (6): Data, DevOps, Documentation, Infrastructure, Programming, Supported Tasks

### Community 159 - "openai.md"
Cohesion: 0.02
Nodes (50): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+42 more)

### Community 160 - "powershell.md"
Cohesion: 0.02
Nodes (52): Signature, Signature, Calls, Signature, Called by, Signature, Calls, Signature (+44 more)

### Community 161 - "qwen.md"
Cohesion: 0.02
Nodes (45): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+37 more)

### Community 162 - "plan_tests.md"
Cohesion: 0.03
Nodes (56): Calls, Signature, Calls, Signature, Calls, Signature, Signature, Signature (+48 more)

### Community 163 - "9. Capacités principales"
Cohesion: 0.40
Nodes (5): 9. Capacités principales, Compréhension de code, Debugging, Génération, Refactoring

### Community 164 - "Session"
Cohesion: 0.16
Nodes (24): FromRow, SqliteRow, Attachment, CompactionRecord, interrupted_plan_from_tasks(), Message, Plan, PlanTask (+16 more)

### Community 165 - "bash.md"
Cohesion: 0.03
Nodes (53): Signature, Signature, Signature, Signature, Signature, Calls, Signature, Signature (+45 more)

### Community 166 - "skill.md"
Cohesion: 0.03
Nodes (53): Signature, Signature, Signature, Signature, Called by, Calls, Signature, Called by (+45 more)

### Community 167 - "handle_chat_key.md"
Cohesion: 0.03
Nodes (58): Calls, Signature, Calls, Signature, Called by, Signature, Called by, Calls (+50 more)

### Community 168 - "types.md"
Cohesion: 0.03
Nodes (43): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+35 more)

### Community 169 - "send_message_with_tools_inner.md"
Cohesion: 0.03
Nodes (51): Called by, Calls, Signature, Called by, Signature, Called by, Calls, Signature (+43 more)

### Community 170 - "next.md"
Cohesion: 0.03
Nodes (45): Signature, Calls, Signature, Called by, Signature, Calls, Signature, Called by (+37 more)

### Community 171 - "src/plan.md"
Cohesion: 0.03
Nodes (38): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+30 more)

### Community 172 - "run_migrations.md"
Cohesion: 0.03
Nodes (48): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+40 more)

### Community 173 - "plan_tool_security_tests.md"
Cohesion: 0.04
Nodes (46): Signature, Calls, Signature, Calls, Signature, Signature, Signature, Calls (+38 more)

### Community 174 - "switch_mode.md"
Cohesion: 0.04
Nodes (49): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+41 more)

### Community 175 - "tui/events.md"
Cohesion: 0.03
Nodes (43): Signature, Signature, Signature, Signature, Signature, Called by, Signature, Called by (+35 more)

### Community 176 - "tui/app.md"
Cohesion: 0.03
Nodes (36): Signature, Signature, Signature, Called by, Signature, Signature, Signature, Signature (+28 more)

### Community 177 - "plan_mode_integration_test.rs"
Cohesion: 0.36
Nodes (13): create_multi_task_plan(), TempDir, Uuid, setup_test_env(), test_end_to_end_plan_creation_and_retrieval(), test_get_most_recent_plan_integration(), test_json_export_import_integration(), test_multiple_concurrent_plans_for_same_session() (+5 more)

### Community 178 - "error_scenarios_test.md"
Cohesion: 0.03
Nodes (37): Signature, Signature, Signature, Called by, Calls, Signature, Signature, Signature (+29 more)

### Community 179 - "task.md"
Cohesion: 0.04
Nodes (36): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+28 more)

### Community 180 - "classes/index.md"
Cohesion: 0.04
Nodes (30): Classes, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+22 more)

### Community 181 - "client.md"
Cohesion: 0.04
Nodes (31): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+23 more)

### Community 182 - "agent/memory.md"
Cohesion: 0.05
Nodes (36): Signature, Signature, Signature, Signature, Called by, Signature, Called by, Calls (+28 more)

### Community 183 - "render_to_string.md"
Cohesion: 0.04
Nodes (37): Calls, Signature, Calls, Signature, Calls, Signature, Calls, Signature (+29 more)

### Community 184 - "edit.md"
Cohesion: 0.04
Nodes (29): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+21 more)

### Community 185 - "cli.md"
Cohesion: 0.04
Nodes (31): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+23 more)

### Community 186 - "todo_write.md"
Cohesion: 0.04
Nodes (28): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+20 more)

### Community 187 - "TaskStatus"
Cohesion: 0.32
Nodes (4): Display, Formatter, Result, TaskStatus

### Community 188 - "tui/error.md"
Cohesion: 0.04
Nodes (30): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+22 more)

### Community 189 - "models.md"
Cohesion: 0.04
Nodes (27): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+19 more)

### Community 190 - "ollama_models.md"
Cohesion: 0.05
Nodes (33): Signature, Signature, Signature, Called by, Calls, Signature, Calls, Signature (+25 more)

### Community 191 - "integration_test.md"
Cohesion: 0.04
Nodes (32): Signature, Called by, Calls, Signature, Calls, Signature, Signature, Signature (+24 more)

### Community 192 - "runner.md"
Cohesion: 0.04
Nodes (33): Signature, Called by, Calls, Signature, Called by, Signature, Called by, Calls (+25 more)

### Community 193 - "TuiEvent"
Cohesion: 0.08
Nodes (36): EventHandler, Default, Instant, Option, PathBuf, String, UnboundedReceiver, UnboundedSender (+28 more)

### Community 194 - "cache.md"
Cohesion: 0.04
Nodes (31): Signature, Signature, Signature, Signature, Calls, Signature, Calls, Signature (+23 more)

### Community 195 - "render/render.md"
Cohesion: 0.05
Nodes (37): Called by, Signature, Called by, Calls, Called by, Calls, Signature, Called by (+29 more)

### Community 196 - "tools/trait.md"
Cohesion: 0.04
Nodes (30): Signature, Signature, Signature, Calls, Signature, Called by, Calls, Signature (+22 more)

### Community 198 - "Architecture Validation"
Cohesion: 0.33
Nodes (5): Architecture Validation, Cycles (0 found), Import Cycles & Orphan Modules (docs/graph/graph.json), Orphan Modules (44 graph-only candidates), Unused Dependencies (cargo-machete)

### Community 199 - "apply_patch.md"
Cohesion: 0.04
Nodes (25): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+17 more)

### Community 200 - "sandbox.md"
Cohesion: 0.04
Nodes (25): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+17 more)

### Community 201 - "NNNN. Short title of the decision"
Cohesion: 0.40
Nodes (4): Consequences, Context, Decision, NNNN. Short title of the decision

### Community 202 - "Module Coupling: cargo-modules vs graphify communities"
Cohesion: 0.40
Nodes (4): Cross-directory module dependencies (rust-analyzer-verified), Directory cohesion (graphify community spread), Layering check against docs/architecture/containers.md, Module Coupling: cargo-modules vs graphify communities

### Community 203 - "save_memory.md"
Cohesion: 0.04
Nodes (28): Signature, Signature, Calls, Signature, Called by, Calls, Signature, Signature (+20 more)

### Community 204 - "src/config.md"
Cohesion: 0.04
Nodes (25): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+17 more)

### Community 205 - "db/retry.md"
Cohesion: 0.05
Nodes (28): Signature, Signature, Signature, Signature, Signature, Called by, Signature, Calls (+20 more)

### Community 206 - "web_fetch.md"
Cohesion: 0.05
Nodes (26): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Called by (+18 more)

### Community 207 - "handle_event.md"
Cohesion: 0.04
Nodes (31): Called by, Signature, Called by, Signature, Called by, Signature, Called by, Calls (+23 more)

### Community 208 - "handle_key_event.md"
Cohesion: 0.05
Nodes (34): Called by, Signature, Called by, Calls, Signature, Called by, Calls, Signature (+26 more)

### Community 209 - "azure.md"
Cohesion: 0.05
Nodes (27): Calls, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+19 more)

### Community 210 - "handle_model_download_key.md"
Cohesion: 0.05
Nodes (31): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+23 more)

### Community 211 - "create_new_session.md"
Cohesion: 0.05
Nodes (30): Called by, Signature, Called by, Calls, Signature, Called by, Calls, Signature (+22 more)

### Community 212 - "repository/session.md"
Cohesion: 0.05
Nodes (25): Signature, Signature, Called by, Signature, Signature, Signature, Signature, Signature (+17 more)

### Community 213 - "anthropic.md"
Cohesion: 0.05
Nodes (22): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+14 more)

### Community 214 - "ollama.md"
Cohesion: 0.05
Nodes (22): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+14 more)

### Community 215 - "key.md"
Cohesion: 0.05
Nodes (27): Calls, Signature, Calls, Signature, Calls, Signature, Calls, Signature (+19 more)

### Community 216 - "notebook.md"
Cohesion: 0.05
Nodes (22): Signature, Signature, Signature, Signature, Signature, Signature, Called by, Signature (+14 more)

### Community 217 - "DummyProvider"
Cohesion: 0.25
Nodes (3): DummyProvider, ProviderResult, ProviderStream

### Community 218 - "Architecture Drift Check"
Cohesion: 0.50
Nodes (3): Architecture Drift Check, Core Components (ARCHITECTURE.md §3) still present in the graph, Top god nodes (GRAPH_REPORT.md) mentioned in ARCHITECTURE.md

### Community 219 - "plan_mode_integration_test.md"
Cohesion: 0.08
Nodes (27): Called by, Calls, Signature, Called by, Calls, Signature, Calls, Signature (+19 more)

### Community 220 - "provider/trait.md"
Cohesion: 0.05
Nodes (21): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+13 more)

### Community 221 - "tools/agent.md"
Cohesion: 0.05
Nodes (22): Signature, Signature, Signature, Signature, Signature, Calls, Signature, Signature (+14 more)

### Community 222 - "tools/context.md"
Cohesion: 0.05
Nodes (21): Signature, Signature, Signature, Signature, Signature, Calls, Signature, Signature (+13 more)

### Community 223 - "handle_plan_key.md"
Cohesion: 0.06
Nodes (28): Called by, Signature, Called by, Calls, Signature, Called by, Calls, Signature (+20 more)

### Community 224 - "set_input_text.md"
Cohesion: 0.06
Nodes (26): Called by, Calls, Signature, Calls, Signature, Called by, Signature, Called by (+18 more)

### Community 225 - "highlight.md"
Cohesion: 0.07
Nodes (25): Called by, Signature, Called by, Signature, Called by, Calls, Signature, Calls (+17 more)

### Community 226 - "llama_cpp.rs"
Cohesion: 0.12
Nodes (27): LlamaBatch, LlamaToken, bash_tool(), build_sampler(), build_sampler_seed_offset_changes_the_resolved_seed(), drain_valid_utf8_empty_buffer_returns_none(), gpu_backend_compiled_in(), maybe_swap_to_constrained_sampler() (+19 more)

### Community 227 - "router.md"
Cohesion: 0.06
Nodes (22): Signature, Signature, Calls, Signature, Signature, Called by, Signature, Calls (+14 more)

### Community 228 - "write.md"
Cohesion: 0.06
Nodes (19): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+11 more)

### Community 229 - "pdf_context.md"
Cohesion: 0.07
Nodes (24): Called by, Calls, Signature, Calls, Signature, Calls, Signature, Signature (+16 more)

### Community 230 - "PlanTool/tool/execute.md"
Cohesion: 0.06
Nodes (25): Calls, Signature, Called by, Calls, Signature, Called by, Called by, Signature (+17 more)

### Community 231 - "render_help.md"
Cohesion: 0.07
Nodes (26): Called by, Signature, Called by, Calls, Signature, Called by, Calls, Signature (+18 more)

### Community 232 - "services/plan.md"
Cohesion: 0.06
Nodes (19): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+11 more)

### Community 233 - "SessionService"
Cohesion: 0.19
Nodes (19): create_test_service(), Message, Option, Result, Self, String, Uuid, Vec (+11 more)

### Community 234 - "with_context.md"
Cohesion: 0.06
Nodes (24): Called by, Calls, Signature, Calls, Signature, Calls, Signature, Called by (+16 more)

### Community 235 - "to_gemini_request.md"
Cohesion: 0.06
Nodes (23): Called by, Signature, Called by, Signature, Called by, Calls, Signature, Calls (+15 more)

### Community 236 - "services.md"
Cohesion: 0.07
Nodes (20): Signature, Calls, Signature, Signature, Signature, Signature, Called by, Signature (+12 more)

### Community 237 - "ollama_download.md"
Cohesion: 0.07
Nodes (20): Signature, Signature, Signature, Called by, Calls, Calls, Signature, Calls (+12 more)

### Community 238 - "plan_tool.md"
Cohesion: 0.06
Nodes (17): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+9 more)

### Community 240 - "CLAUDE.md"
Cohesion: 0.12
Nodes (15): Architecture Overview, Architecture Validation Scripts, Building and Running, Code Quality, Common Development Commands, Core Module Structure, Database Operations, Database Schema (+7 more)

### Community 242 - "read.md"
Cohesion: 0.06
Nodes (17): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+9 more)

### Community 243 - "logging.md"
Cohesion: 0.07
Nodes (19): Signature, Signature, Signature, Signature, Signature, Called by, Signature, Signature (+11 more)

### Community 244 - "tool_call_from_content.md"
Cohesion: 0.07
Nodes (22): Called by, Signature, Called by, Signature, Calls, Signature, Calls, Signature (+14 more)

### Community 245 - "tui/render.md"
Cohesion: 0.06
Nodes (19): Calls, Signature, Calls, Signature, Signature, Signature, Signature, Signature (+11 more)

### Community 246 - "file_read_cache.md"
Cohesion: 0.07
Nodes (19): Signature, Signature, Signature, Calls, Signature, Signature, Signature, Called by (+11 more)

### Community 247 - "glob.md"
Cohesion: 0.06
Nodes (17): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+9 more)

### Community 248 - "grep.md"
Cohesion: 0.06
Nodes (17): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+9 more)

### Community 249 - "http.md"
Cohesion: 0.07
Nodes (18): Signature, Signature, Signature, Signature, Calls, Signature, Signature, Signature (+10 more)

### Community 250 - "from_ollama_response.md"
Cohesion: 0.09
Nodes (21): Calls, Signature, Calls, Signature, Calls, Signature, Calls, Signature (+13 more)

### Community 251 - "llama_cpp_models.rs"
Cohesion: 0.14
Nodes (23): delete_model(), download_model(), download_model_rejects_a_checksum_mismatch_and_cleans_up(), download_model_writes_the_file_and_reports_progress(), DownloadProgress, fetch_hf_lfs_sha256(), list_local_models(), list_local_models_on_nonexistent_dir_returns_empty_not_error() (+15 more)

### Community 252 - "tool_call_recovery.rs"
Cohesion: 0.13
Nodes (29): bash_tool(), brace_depth_at(), commits_to_an_offered_tool_call(), commits_to_an_offered_tool_call_accepts_the_top_level_name_after_other_keys(), commits_to_an_offered_tool_call_does_not_scan_past_the_window(), commits_to_an_offered_tool_call_recognizes_compact_and_spaced_name_keys(), commits_to_an_offered_tool_call_rejects_a_fenced_block(), commits_to_an_offered_tool_call_rejects_a_nested_name_field() (+21 more)

### Community 253 - "ask_user.md"
Cohesion: 0.07
Nodes (17): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Calls (+9 more)

### Community 255 - "doc_parser.md"
Cohesion: 0.07
Nodes (16): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+8 more)

### Community 257 - "ls.md"
Cohesion: 0.07
Nodes (18): Signature, Signature, Called by, Calls, Signature, Called by, Signature, Signature (+10 more)

### Community 258 - "repository/message.md"
Cohesion: 0.07
Nodes (17): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+9 more)

### Community 259 - "parse_fallback_tool_calls.md"
Cohesion: 0.07
Nodes (21): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+13 more)

### Community 260 - "compact.md"
Cohesion: 0.08
Nodes (20): Signature, Called by, Calls, Signature, Calls, Signature, Calls, Signature (+12 more)

### Community 269 - "web_search.md"
Cohesion: 0.07
Nodes (16): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+8 more)

### Community 382 - "sandbox.rs"
Cohesion: 0.14
Nodes (20): absolute_path_outside_root_denied(), absolute_path_to_nonexistent_file_in_subdir_allowed(), absolute_path_to_nonexistent_file_inside_root_allowed(), absolute_path_to_nonexistent_file_outside_root_still_denied(), absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed(), AllowAll, and_policy_short_circuits_on_deny(), check_path() (+12 more)

### Community 411 - "finish.md"
Cohesion: 0.08
Nodes (19): Signature, Calls, Signature, Calls, Signature, Calls, Signature, Calls (+11 more)

### Community 412 - "repository/file.md"
Cohesion: 0.07
Nodes (16): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+8 more)

### Community 413 - "repository/memory.md"
Cohesion: 0.08
Nodes (19): Signature, Calls, Signature, Called by, Calls, Signature, Signature, Called by (+11 more)

### Community 414 - "record.md"
Cohesion: 0.09
Nodes (19): Calls, Signature, Calls, Signature, Calls, Signature, Calls, Signature (+11 more)

### Community 415 - "render_approval.md"
Cohesion: 0.07
Nodes (20): Called by, Signature, Called by, Signature, Called by, Signature, Called by, Calls (+12 more)

### Community 416 - "from_qwen_response.md"
Cohesion: 0.07
Nodes (19): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+11 more)

### Community 417 - "repository/plan.md"
Cohesion: 0.08
Nodes (14): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+6 more)

### Community 418 - "from_config.md"
Cohesion: 0.09
Nodes (18): Calls, Signature, Called by, Calls, Signature, Called by, Calls, Signature (+10 more)

### Community 419 - "add_message.md"
Cohesion: 0.09
Nodes (17): Calls, Signature, Called by, Calls, Signature, Called by, Signature, Calls (+9 more)

### Community 420 - "to_qwen_request.md"
Cohesion: 0.08
Nodes (17): Called by, Signature, Called by, Signature, Called by, Calls, Signature, Calls (+9 more)

### Community 421 - "with_tool_parser.md"
Cohesion: 0.08
Nodes (17): Called by, Signature, Called by, Calls, Signature, Called by, Signature, Calls (+9 more)

### Community 422 - "setup_test_service.md"
Cohesion: 0.09
Nodes (17): Called by, Signature, Called by, Calls, Signature, Calls, Signature, Calls (+9 more)

### Community 423 - "db.md"
Cohesion: 0.08
Nodes (13): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+5 more)

### Community 424 - "provider/retry.md"
Cohesion: 0.08
Nodes (13): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+5 more)

### Community 425 - "ssrf_guard.md"
Cohesion: 0.08
Nodes (13): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+5 more)

### Community 426 - "cmd_chat.md"
Cohesion: 0.08
Nodes (17): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+9 more)

### Community 427 - "cli/run.md"
Cohesion: 0.08
Nodes (17): Called by, Signature, Called by, Calls, Signature, Called by, Signature, Called by (+9 more)

### Community 428 - "plan_crash_recovery_test.md"
Cohesion: 0.11
Nodes (16): Called by, Signature, Calls, Signature, Called by, Signature, Signature, Calls (+8 more)

### Community 429 - "default_local.md"
Cohesion: 0.08
Nodes (16): Called by, Signature, Calls, Signature, Calls, Signature, Calls, Signature (+8 more)

### Community 430 - "test_to_ollama_request_maps_common_fields.md"
Cohesion: 0.11
Nodes (16): Calls, Signature, Calls, Signature, Calls, Signature, Called by, Signature (+8 more)

### Community 431 - "dashscope_intl.md"
Cohesion: 0.08
Nodes (16): Called by, Signature, Calls, Signature, Calls, Signature, Calls, Signature (+8 more)

### Community 432 - "compaction_test.md"
Cohesion: 0.10
Nodes (16): Called by, Calls, Signature, Calls, Signature, Calls, Signature, Calls (+8 more)

### Community 433 - "interfaces/index.md"
Cohesion: 0.08
Nodes (13): Knowledge Base, Interfaces, Signature, Signature, Signature, Signature, Signature, Signature (+5 more)

### Community 434 - "registry.rs"
Cohesion: 0.20
Nodes (17): an_exact_match_wins_over_an_alias_entry(), execute_evaluates_policy_against_the_canonical_name_not_the_alias(), execute_reports_not_found_using_the_original_unresolved_name(), execute_resolves_an_alias_name_to_the_registered_tool(), get_resolves_a_known_alias_to_the_registered_canonical_tool(), has_tool_is_false_for_an_alias_whose_target_is_not_registered(), is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias(), register_mcp_server_with_nonexistent_command_fails_gracefully() (+9 more)

### Community 435 - "code_exec.md"
Cohesion: 0.09
Nodes (13): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Calls (+5 more)

### Community 436 - "OllamaProvider/provider/stream.md"
Cohesion: 0.09
Nodes (16): Called by, Signature, Called by, Calls, Signature, Calls, Signature, Called by (+8 more)

### Community 437 - "MarkdownRenderer"
Cohesion: 0.15
Nodes (10): CodeBlockKind, CowStr, Span, last_code_block(), MarkdownRenderer, Option, Self, String (+2 more)

### Community 438 - "parse_patch.md"
Cohesion: 0.09
Nodes (15): Calls, Signature, Calls, Signature, Calls, Signature, Called by, Calls (+7 more)

### Community 439 - "event_sender.md"
Cohesion: 0.09
Nodes (16): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+8 more)

### Community 440 - "parse_markdown.md"
Cohesion: 0.09
Nodes (15): Called by, Calls, Signature, Calls, Signature, Calls, Signature, Calls (+7 more)

### Community 441 - "repository/plan.rs"
Cohesion: 0.37
Nodes (17): create_test_plan(), setup_test_db(), test_multiple_sessions_multiple_plans(), test_plan_create(), test_plan_delete(), test_plan_find_by_id(), test_plan_find_by_session_id(), test_plan_risks_serialization() (+9 more)

### Community 442 - "ApplyPatchTool/tool/execute.md"
Cohesion: 0.11
Nodes (14): Signature, Calls, Signature, Signature, Calls, Signature, Called by, Calls (+6 more)

### Community 443 - "LlamaCppProvider"
Cohesion: 0.11
Nodes (10): Sender, InferenceJob, LlamaCppProvider, Debug, Formatter, PathBuf, ProviderStream, UnboundedReceiver (+2 more)

### Community 444 - "parse_markdown"
Cohesion: 0.19
Nodes (16): markdown_escapes_backslash_before_punctuation(), parse_markdown(), parse_plain_text(), plain_text_keeps_markdown_syntax_literal(), plain_text_keeps_windows_path_backslashes(), plain_text_preserves_line_structure(), rendered_text(), Line (+8 more)

### Community 445 - "config/update.md"
Cohesion: 0.10
Nodes (11): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Signature (+3 more)

### Community 446 - "markdown.md"
Cohesion: 0.10
Nodes (12): Signature, Signature, Signature, Signature, Called by, Signature, Called by, Signature (+4 more)

### Community 447 - "cmd_ollama.md"
Cohesion: 0.10
Nodes (14): Calls, Signature, Called by, Signature, Called by, Calls, Signature, Called by (+6 more)

### Community 448 - "HttpClientTool/tool/execute.md"
Cohesion: 0.12
Nodes (14): Calls, Signature, Called by, Calls, Signature, Calls, Signature, Called by (+6 more)

### Community 449 - "ollama_download.rs"
Cohesion: 0.18
Nodes (17): build_ollama_provider(), fetch_installed_models(), filter_suggestions(), filter_suggestions_empty_query_returns_all_deduped(), filter_suggestions_includes_ornith(), filter_suggestions_matches_substring_case_insensitive(), ModelPullProgress, Arc (+9 more)

### Community 450 - "agent/context.md"
Cohesion: 0.11
Nodes (10): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Contains (+2 more)

### Community 451 - "parse.md"
Cohesion: 0.11
Nodes (13): Called by, Signature, Called by, Calls, Signature, Calls, Signature, Calls (+5 more)

### Community 452 - "check_path.md"
Cohesion: 0.11
Nodes (13): Calls, Signature, Calls, Signature, Called by, Calls, Signature, Calls (+5 more)

### Community 453 - "`llama-cpp-2` Integration Plan"
Cohesion: 0.11
Nodes (19): 0.0 Implementation status (honest, updated as phases land), 0.1 Go/No-Go status inherited from the prior feasibility study, 0. Summary for reviewers, 10. Open questions to settle before implementation, 11. What does NOT change, 12. Test plan, 1. Objective, 2. Why add this, given Ollama already covers local inference? (+11 more)

### Community 454 - "Spécification d'évaluation : chargement direct de modèles GGUF via `llama-cpp-2`"
Cohesion: 0.11
Nodes (19): 0. Résumé exécutif, 1. Contexte : pourquoi cette évaluation, 2.1 Composition et provenance, 2.2 Licence, 2.3 Maturité, 2.4 Fonctionnalités couvertes (features Cargo), 2. `llama-cpp-2` : ce que c'est, 3. Alignement avec l'architecture Crustly actuelle (+11 more)

### Community 455 - "Stratégie de différenciation : Crustly face à OpenCode"
Cohesion: 0.11
Nodes (18): 0. Résumé exécutif, 1.1 Échelle et modèle, 1.2 Architecture, 1.3 Fonctionnalités notables, 1.4 Modèle de permissions et de sécurité — le point le plus important pour cette étude, 1. Profil concurrentiel : OpenCode, 2. Ce que Crustly ne doit pas chasser (anti-objectifs), 3.1 Efficacité ressources — mesurable, pas seulement revendiquée (+10 more)

### Community 456 - "⚙️ Using Crustly with llama.cpp (in-process, no server)"
Cohesion: 0.11
Nodes (18): A crash takes down the whole Crustly process, Build fails with a cmake/compiler error, Build requirements (read this before enabling), Configuration, Getting a model, GPU acceleration, Grammar-constrained tool calling (optional), "llama.cpp model file not found" (+10 more)

### Community 457 - "crabrace.md"
Cohesion: 0.11
Nodes (10): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Contains (+2 more)

### Community 458 - "Config/load.md"
Cohesion: 0.11
Nodes (12): Calls, Signature, Called by, Signature, Called by, Signature, Called by, Signature (+4 more)

### Community 459 - "ollama_provider_from_config.md"
Cohesion: 0.11
Nodes (13): Called by, Calls, Signature, Calls, Signature, Called by, Calls, Signature (+5 more)

### Community 460 - "retry_with_backoff.md"
Cohesion: 0.11
Nodes (12): Called by, Signature, Calls, Signature, Calls, Signature, Calls, Signature (+4 more)

### Community 461 - "aliases.md"
Cohesion: 0.11
Nodes (10): Signature, Signature, Signature, Signature, Signature, Signature, Signature, Contains (+2 more)

### Community 462 - "context_with_file.md"
Cohesion: 0.11
Nodes (12): Called by, Signature, Calls, Calls, Signature, Signature, Calls, Signature (+4 more)

### Community 463 - "make_root.md"
Cohesion: 0.11
Nodes (12): Calls, Signature, Calls, Signature, Called by, Signature, Calls, Signature (+4 more)

### Community 464 - "logo.md"
Cohesion: 0.12
Nodes (11): Signature, Signature, Called by, Signature, Signature, Signature, Calls, Signature (+3 more)

### Community 465 - "parse_plain_text.md"
Cohesion: 0.12
Nodes (12): Calls, Signature, Called by, Signature, Calls, Signature, Calls, Signature (+4 more)

### Community 466 - "render_chat.md"
Cohesion: 0.11
Nodes (13): Called by, Signature, Called by, Calls, Signature, Called by, Signature, Called by (+5 more)

### Community 467 - "llama_cpp_grammar.rs"
Cohesion: 0.26
Nodes (17): ParserFactory, bash_tool(), build_parser_factory(), build_tool_call_sampler(), each_variant_pins_the_tool_name_and_embeds_its_input_schema(), empty_tool_list_produces_an_empty_oneof(), one_factory_builds_multiple_independent_grammars(), read_tool() (+9 more)

### Community 468 - "src/app.md"
Cohesion: 0.12
Nodes (10): Signature, Signature, Signature, Signature, Signature, Calls, Signature, Contains (+2 more)

### Community 469 - "parallel_tool_dispatch.md"
Cohesion: 0.14
Nodes (12): Calls, Signature, Called by, Signature, Called by, Signature, Called by, Calls (+4 more)

### Community 470 - "apply_env_overrides.md"
Cohesion: 0.12
Nodes (12): Called by, Calls, Signature, Called by, Calls, Signature, Calls, Signature (+4 more)

### Community 471 - "interrupted_plan_from_tasks.md"
Cohesion: 0.12
Nodes (12): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Signature (+4 more)

### Community 472 - "apply_hunks.md"
Cohesion: 0.12
Nodes (12): Called by, Calls, Calls, Signature, Calls, Signature, Calls, Signature (+4 more)

### Community 473 - "list_files_for_session.md"
Cohesion: 0.12
Nodes (12): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Signature (+4 more)

### Community 474 - "ServiceContext"
Cohesion: 0.26
Nodes (8): Pool, create_test_pool(), Arc, Self, ServiceContext, ServiceManager, test_service_context_creation(), test_service_manager_creation()

### Community 475 - "pdf_context.rs"
Cohesion: 0.21
Nodes (14): augment_message_with_pdf(), augment_returns_original_on_extraction_failure(), augment_returns_original_when_no_pdf(), detects_absolute_pdf_token(), detects_relative_pdf_token(), extract_pdf_text(), looks_like_pdf_path(), returns_none_for_missing_file() (+6 more)

### Community 476 - "Crustly Performance Improvement Plan"
Cohesion: 0.12
Nodes (15): 1.1 Expose the local-inference knobs that actually move throughput, 1.2 Reconcile Crabrace's documented role with what it actually does, 1.3 Tokenizer-accuracy check for compaction timing, 2.1 Stop deep-cloning the full message history on every tool-loop iteration, 2.2 Real incremental streaming for Ollama instead of buffer-then-replay, 2.3 Avoid re-parsing the entire response as markdown on every redraw, 2.4 Stop reloading and re-tokenizing full session history every turn, Crustly Performance Improvement Plan (+7 more)

### Community 477 - "utils/retry.md"
Cohesion: 0.12
Nodes (9): Signature, Signature, Signature, Signature, Signature, Signature, Contains, Imports (+1 more)

### Community 478 - "plan_from_db.md"
Cohesion: 0.13
Nodes (11): Calls, Signature, Calls, Signature, Called by, Signature, Called by, Signature (+3 more)

### Community 479 - "from_gemini_response.md"
Cohesion: 0.12
Nodes (11): Called by, Calls, Signature, Called by, Signature, Calls, Signature, Calls (+3 more)

### Community 480 - "is_success.md"
Cohesion: 0.12
Nodes (11): Called by, Signature, Called by, Calls, Signature, Calls, Signature, Calls (+3 more)

### Community 481 - "to_ollama_request.md"
Cohesion: 0.12
Nodes (11): Calls, Signature, Called by, Calls, Signature, Calls, Signature, Calls (+3 more)

### Community 482 - "extract_thinking.md"
Cohesion: 0.12
Nodes (11): Called by, Calls, Signature, Signature, Called by, Calls, Signature, Calls (+3 more)

### Community 483 - "parse_hermes_tool_calls.md"
Cohesion: 0.12
Nodes (11): Called by, Calls, Signature, Calls, Signature, Calls, Signature, Calls (+3 more)

### Community 484 - "flush_current_line.md"
Cohesion: 0.13
Nodes (12): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+4 more)

### Community 485 - "render_message_lines.md"
Cohesion: 0.12
Nodes (11): Called by, Signature, Calls, Signature, Called by, Calls, Signature, Called by (+3 more)

### Community 486 - "render_model_download.md"
Cohesion: 0.12
Nodes (12): Called by, Calls, Called by, Calls, Signature, Called by, Calls, Signature (+4 more)

### Community 487 - "agent/compaction.rs"
Cohesion: 0.22
Nodes (14): compact(), compaction_atomicity_db_failure_leaves_context_unchanged(), compaction_integration_preserves_last_10_turns(), compaction_never_splits_a_tool_use_result_pair(), CompactionRecord, message_has_tool_result(), DateTime, Message (+6 more)

### Community 488 - "Config/default/default.md"
Cohesion: 0.13
Nodes (10): Calls, Signature, Calls, Signature, Called by, Signature, Called by, Signature (+2 more)

### Community 489 - "ToolCacheConfig/default/default.md"
Cohesion: 0.13
Nodes (10): Called by, Signature, Called by, Signature, Called by, Signature, Called by, Signature (+2 more)

### Community 490 - "init_minimal_logging.md"
Cohesion: 0.13
Nodes (11): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+3 more)

### Community 491 - "get_most_recent_plan.md"
Cohesion: 0.13
Nodes (11): Called by, Calls, Signature, Calls, Signature, Called by, Calls, Signature (+3 more)

### Community 492 - "history_prev.md"
Cohesion: 0.13
Nodes (11): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Signature (+3 more)

### Community 493 - "dialogs.md"
Cohesion: 0.15
Nodes (10): Called by, Signature, Signature, Calls, Signature, Calls, Signature, Contains (+2 more)

### Community 494 - "handle_end_tag.md"
Cohesion: 0.13
Nodes (11): Called by, Calls, Signature, Called by, Signature, Called by, Calls, Signature (+3 more)

### Community 495 - "render_plan.md"
Cohesion: 0.13
Nodes (11): Called by, Calls, Called by, Calls, Signature, Called by, Signature, Signature (+3 more)

### Community 496 - "provider/error.md"
Cohesion: 0.14
Nodes (8): Signature, Signature, Signature, Signature, Signature, Contains, Imports, Member of

### Community 497 - "services/file.md"
Cohesion: 0.14
Nodes (8): Signature, Signature, Signature, Signature, Signature, Contains, Imports, Member of

### Community 498 - "estimate_tokens.md"
Cohesion: 0.14
Nodes (10): Called by, Calls, Signature, Called by, Calls, Signature, Calls, Signature (+2 more)

### Community 499 - "is_blocked_ip.md"
Cohesion: 0.15
Nodes (10): Called by, Calls, Signature, Called by, Signature, Called by, Calls, Signature (+2 more)

### Community 500 - "4. Architecture"
Cohesion: 0.14
Nodes (14): 4.0 Codebase precedents (confirmed via `docs/graph/graph.json` and `knowledge/`), 4.10.1 Knowledge-graph staleness note (methodology, not a code issue), 4.10 Compatibility with `ModelRouter` / prompt-tier auto-routing, 4.11 Process isolation & security surface, 4.1 Dependencies (`Cargo.toml`), 4.2 Build requirements (must be documented, not hidden), 4.3 New module: `src/llm/provider/llama_cpp.rs`, 4.4 Why a dedicated worker thread, not `spawn_blocking` per request (+6 more)

### Community 501 - "run_stream"
Cohesion: 0.46
Nodes (14): LlamaChatTemplate, LlamaContext, LlamaModel, build_grammar_env(), build_prompt(), decode_one_more(), dispatch_job(), prepare_generation() (+6 more)

### Community 502 - "parse_patch"
Cohesion: 0.20
Nodes (14): ApplyPatchInput, FileOp, find_subsequence(), Hunk, HunkLine, parse_add_file_collects_plus_prefixed_lines(), parse_multiple_file_ops_in_one_patch(), parse_patch() (+6 more)

### Community 503 - "Interfaces"
Cohesion: 0.15
Nodes (12): Contents, crustly, Documentation, Interfaces, Packages, PermissionPolicy, PoolExt, Provider (+4 more)

### Community 504 - "cmd_logs.md"
Cohesion: 0.15
Nodes (9): Called by, Calls, Signature, Called by, Signature, Called by, Signature, Calls (+1 more)

### Community 505 - "plan_to_db.md"
Cohesion: 0.15
Nodes (9): Calls, Signature, Called by, Signature, Called by, Calls, Signature, Calls (+1 more)

### Community 506 - "with_think.md"
Cohesion: 0.15
Nodes (9): Calls, Signature, Called by, Calls, Signature, Called by, Signature, Calls (+1 more)

### Community 507 - "DocParserTool/tool/execute.md"
Cohesion: 0.15
Nodes (9): Called by, Signature, Called by, Calls, Signature, Called by, Signature, Calls (+1 more)

### Community 508 - "normalize_path.md"
Cohesion: 0.17
Nodes (9): Calls, Signature, Called by, Signature, Calls, Signature, Called by, Calls (+1 more)

### Community 509 - "import_from_json.md"
Cohesion: 0.15
Nodes (9): Called by, Calls, Signature, Calls, Signature, Calls, Signature, Calls (+1 more)

### Community 510 - "13. Phasing"
Cohesion: 0.15
Nodes (13): 13. Phasing, Phase 0 — Feasibility spike (no user-facing code) ✅ Done — see §0.0, Phase 10 — Documentation & rollout, Phase 1 — MVP provider (CPU only, non-streaming) ✅ Done — see §0.0, Phase 2 — Streaming, Phase 3 — Sampling, context, chat templates, Phase 4 — Tool calling, Phase 4b — Grammar-constrained tool calling (`llguidance`, optional) (+5 more)

### Community 511 - "DummyProvider"
Cohesion: 0.17
Nodes (6): DummyProvider, Option, ProviderResult, ProviderStream, String, Vec

### Community 512 - "CrabraceConfig/default/default.md"
Cohesion: 0.17
Nodes (8): Calls, Signature, Called by, Signature, Called by, Signature, Called by, Signature

### Community 513 - "update_task_status.md"
Cohesion: 0.17
Nodes (8): Called by, Signature, Calls, Signature, Calls, Signature, Calls, Signature

### Community 514 - "AnthropicProvider/provider/complete.md"
Cohesion: 0.17
Nodes (8): Called by, Signature, Calls, Signature, Calls, Signature, Called by, Signature

### Community 515 - "is_local.md"
Cohesion: 0.17
Nodes (8): Calls, Signature, Called by, Signature, Calls, Signature, Calls, Signature

### Community 516 - "parse_docx.md"
Cohesion: 0.17
Nodes (9): Called by, Calls, Signature, Called by, Calls, Signature, Called by, Calls (+1 more)

### Community 517 - "test_task_failure.md"
Cohesion: 0.15
Nodes (8): Calls, Signature, Calls, Signature, Called by, Signature, Called by, Signature

### Community 518 - "get_file.md"
Cohesion: 0.17
Nodes (8): Called by, Signature, Called by, Signature, Calls, Signature, Calls, Signature

### Community 519 - "classify_tier.md"
Cohesion: 0.17
Nodes (8): Called by, Signature, Calls, Signature, Calls, Signature, Calls, Signature

### Community 520 - "render_splash.md"
Cohesion: 0.18
Nodes (9): Called by, Calls, Called by, Calls, Signature, Signature, Contains, Imports (+1 more)

### Community 521 - "model_routing_test.md"
Cohesion: 0.17
Nodes (7): Signature, Signature, Signature, Signature, Contains, Imports, Member of

### Community 522 - "plan_autorun_test.md"
Cohesion: 0.17
Nodes (7): Signature, Signature, Signature, Signature, Contains, Imports, Member of

### Community 523 - "v0.5.2 — Local-Model Reliability & Per-Model Tuning"
Cohesion: 0.18
Nodes (11): Approval That Means Yes, Bug Fixes, Earned Task Completion, `--model` CLI Flag, Per-Model Ollama Settings, Per-Model `think` Control, Qwen3-Coder-Next & Qwen3.6-27B Support, Reasoning-Only Answers Surfaced, Not Blank (+3 more)

### Community 524 - "🔒 Crustly's Security & Permission Model"
Cohesion: 0.18
Nodes (11): Bash command allowlisting — resistant to operator smuggling, Configuration, 🔒 Crustly's Security & Permission Model, How this compares to other terminal AI agents, Path boundary enforcement, Plan Mode approval gating, The built-in rules, The core idea: a composable policy engine, not a single on/off switch (+3 more)

### Community 525 - "prompt_analyzer.md"
Cohesion: 0.18
Nodes (7): Signature, Signature, Calls, Signature, Contains, Imports, Member of

### Community 526 - "merge_from_file.md"
Cohesion: 0.18
Nodes (8): Called by, Calls, Signature, Called by, Called by, Calls, Signature, Signature

### Community 527 - "get_file_required.md"
Cohesion: 0.20
Nodes (8): Called by, Calls, Signature, Called by, Calls, Signature, Calls, Signature

### Community 528 - "String"
Cohesion: 0.27
Nodes (9): Any, drain_valid_utf8(), drain_valid_utf8_multiple_tokens_reassemble_correctly(), drain_valid_utf8_never_panics_on_arbitrary_bytes(), panic_message(), panic_to_provider_error(), Send, String (+1 more)

### Community 530 - "src/error.md"
Cohesion: 0.20
Nodes (6): Signature, Signature, Signature, Contains, Imports, Member of

### Community 531 - "to_policy.md"
Cohesion: 0.20
Nodes (7): Calls, Signature, Calls, Signature, Called by, Calls, Signature

### Community 532 - "find_file_by_path.md"
Cohesion: 0.20
Nodes (7): Called by, Signature, Called by, Calls, Signature, Calls, Signature

### Community 533 - "task_to_db.md"
Cohesion: 0.20
Nodes (7): Called by, Signature, Called by, Signature, Called by, Calls, Signature

### Community 534 - "task_from_db.md"
Cohesion: 0.20
Nodes (7): Called by, Signature, Called by, Signature, Called by, Calls, Signature

### Community 535 - "trim_to_fit.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Called by, Signature, Calls, Signature

### Community 536 - "token_count.md"
Cohesion: 0.20
Nodes (7): Calls, Signature, Called by, Calls, Calls, Signature, Signature

### Community 537 - "parse_anthropic_sse_stream.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Calls, Signature, Calls, Signature

### Community 538 - "list_models.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Calls, Signature, Signature, Calls, Signature

### Community 539 - "stream_events_from_buffered_content.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Calls, Signature, Calls, Signature

### Community 540 - "extract_retry_after.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Called by, Signature, Calls, Signature

### Community 541 - "parse_html.md"
Cohesion: 0.20
Nodes (7): Called by, Signature, Called by, Calls, Signature, Calls, Signature

### Community 542 - "search_file.md"
Cohesion: 0.20
Nodes (7): Called by, Signature, Called by, Calls, Signature, Calls, Signature

### Community 543 - "skip.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Calls, Signature, Called by, Signature

### Community 544 - "setup_from_cli.md"
Cohesion: 0.20
Nodes (7): Called by, Signature, Called by, Calls, Signature, Calls, Signature

### Community 545 - "call_tool.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Calls, Signature, Calls, Signature

### Community 546 - "progress_percentage.md"
Cohesion: 0.20
Nodes (7): Called by, Signature, Called by, Calls, Signature, Calls, Signature

### Community 547 - "copy_last_response_to_clipboard.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Called by, Signature, Called by, Signature

### Community 548 - "switch_provider_to_ollama_model.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Calls, Signature, Calls, Signature

### Community 549 - "analyze_and_transform.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Calls, Signature, Calls, Signature

### Community 550 - "retry/retry.md"
Cohesion: 0.20
Nodes (7): Called by, Calls, Signature, Calls, Signature, Calls, Signature

### Community 551 - "src/utils.md"
Cohesion: 0.20
Nodes (6): Signature, Signature, Signature, Contains, Imports, Member of

### Community 552 - "ApplyPatchTool"
Cohesion: 0.22
Nodes (4): ApplyPatchTool, Result, Tool, Value

### Community 553 - "normalize_path"
Cohesion: 0.38
Nodes (7): DenyPathPrefixRule, normalize_path(), PathBoundaryRule, resolve_existing_prefix(), Path, PathBuf, strip_verbatim_prefix()

### Community 554 - "compaction_test.rs"
Cohesion: 0.38
Nodes (8): build_context(), compaction_fails_gracefully_with_insufficient_turns(), compaction_preserves_last_10_turns(), compaction_writes_one_record_to_db(), create_session(), Message, Uuid, text_message()

### Community 555 - "MemoryConfig/default/default.md"
Cohesion: 0.22
Nodes (6): Called by, Signature, Called by, Signature, Calls, Signature

### Community 556 - "get_task_mut.md"
Cohesion: 0.22
Nodes (6): Calls, Signature, Calls, Signature, Called by, Signature

### Community 557 - "is_vision_model.md"
Cohesion: 0.22
Nodes (6): Called by, Signature, Calls, Signature, Calls, Signature

### Community 558 - "map_ollama_error.md"
Cohesion: 0.22
Nodes (6): Called by, Signature, Calls, Signature, Calls, Signature

### Community 559 - "with_num_ctx.md"
Cohesion: 0.22
Nodes (6): Called by, Signature, Calls, Signature, Calls, Signature

### Community 560 - "to_ollama_tool.md"
Cohesion: 0.22
Nodes (6): Called by, Calls, Signature, Calls, Signature, Signature

### Community 561 - "mock_sse_server.md"
Cohesion: 0.22
Nodes (6): Called by, Signature, Calls, Signature, Calls, Signature

### Community 562 - "build_client.md"
Cohesion: 0.22
Nodes (6): Called by, Signature, Calls, Signature, Calls, Signature

### Community 563 - "approval_dialog_details_view_shows_pretty_printed_json.md"
Cohesion: 0.22
Nodes (6): Calls, Signature, Calls, Signature, Called by, Signature

### Community 564 - "test_preset_configs.md"
Cohesion: 0.22
Nodes (6): Called by, Signature, Called by, Signature, Calls, Signature

### Community 565 - "4. Étude complète des bénéfices"
Cohesion: 0.22
Nodes (9): 4.1 Alignement avec le positionnement produit de Crustly, 4.2 Élimination de la dépendance à un service externe, 4.3 Fiabilité du tool-calling via génération contrainte par grammaire, 4.4 Support multimodal natif (`mtmd`), 4.5 Contrôle total du cycle de vie du modèle, 4.6 Onboarding utilisateur simplifié, 4.7 Différenciation compétitive, 4.8 Tableau récapitulatif des bénéfices (+1 more)

### Community 566 - "build_approval_callback"
Cohesion: 0.25
Nodes (8): auto_mode_bypasses_approval(), build_approval_callback(), ApprovalCallback, Arc, Mutex, UnboundedSender, PlanExecMode, PlanModeConfig

### Community 567 - "repository/compaction.md"
Cohesion: 0.25
Nodes (5): Signature, Signature, Contains, Imports, Member of

### Community 568 - "model_hints.md"
Cohesion: 0.25
Nodes (5): Signature, Signature, Contains, Imports, Member of

### Community 569 - "mcp_contract_test.md"
Cohesion: 0.25
Nodes (5): Signature, Signature, Contains, Imports, Member of

### Community 570 - "get_provider.md"
Cohesion: 0.29
Nodes (5): Called by, Calls, Signature, Calls, Signature

### Community 571 - "ProviderUpdater/update.md"
Cohesion: 0.29
Nodes (5): Calls, Called by, Calls, Signature, Signature

### Community 572 - "row_to_plan_task.md"
Cohesion: 0.29
Nodes (5): Calls, Signature, Called by, Calls, Signature

### Community 573 - "strip_html_tags.md"
Cohesion: 0.29
Nodes (5): Called by, Calls, Signature, Calls, Signature

### Community 574 - "get_or_create_file.md"
Cohesion: 0.29
Nodes (5): Called by, Calls, Signature, Calls, Signature

### Community 575 - "is_file_tracked.md"
Cohesion: 0.29
Nodes (5): Called by, Calls, Signature, Calls, Signature

### Community 576 - "syntect_style_to_ratatui.md"
Cohesion: 0.29
Nodes (5): Called by, Calls, Signature, Called by, Signature

### Community 577 - ".extract_thinking"
Cohesion: 0.33
Nodes (5): find_after(), find_after_returns_an_absolute_offset_not_a_relative_one(), test_sampling_defaults_qwen3_thinking(), test_thinking_extraction(), test_thinking_extraction_out_of_order_tags_does_not_panic()

### Community 578 - "agent/error.md"
Cohesion: 0.33
Nodes (4): Signature, Contains, Imports, Member of

### Community 579 - "fetch_providers.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 580 - "health_check.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 581 - "default_ollama_host.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 582 - "should_update.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 583 - "test_update_result_failure.md"
Cohesion: 0.33
Nodes (4): Calls, Signature, Called by, Signature

### Community 584 - "format_hermes_tools.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 585 - "format_native_qwen_result.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 586 - "format_native_qwen_tools.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 587 - "with_thinking_budget.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 588 - "OrPolicy/permissionpolicy/evaluate.md"
Cohesion: 0.33
Nodes (4): Calls, Signature, Called by, Signature

### Community 589 - "reject.md"
Cohesion: 0.33
Nodes (4): Calls, Signature, Called by, Signature

### Community 590 - "dependencies_satisfied.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 591 - "can_retry.md"
Cohesion: 0.33
Nodes (4): Calls, Signature, Called by, Signature

### Community 592 - "advance.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 593 - "count_files_in_session.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 594 - "test_delete_files_for_session.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 595 - "supported_languages.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 596 - "build_keyword_regex.md"
Cohesion: 0.33
Nodes (4): Called by, Signature, Calls, Signature

### Community 598 - "TestError"
Cohesion: 0.40
Nodes (5): RetryableError, Display, Formatter, String, TestError

### Community 599 - "create_pool_with_schema"
Cohesion: 0.60
Nodes (5): create_pool_with_schema(), fts_search_finds_symbol_by_partial_name(), index_and_query_provider_trait(), index_file_twice_no_duplicate(), index_nonexistent_file_returns_error()

### Community 600 - "benchmark-vs-opencode.sh"
Cohesion: 0.70
Nodes (4): log(), manual_timing_loop(), benchmark-vs-opencode.sh script, usage()

### Community 601 - "render_splash"
Cohesion: 0.70
Nodes (4): render_splash(), render_splash_content(), Frame, Rect

### Community 602 - "parse_xml.md"
Cohesion: 0.50
Nodes (3): Called by, Calls, Signature

### Community 603 - "handle_rule.md"
Cohesion: 0.50
Nodes (3): Called by, Calls, Signature

## Knowledge Gaps
- **7924 isolated node(s):** `check_plan_debug.sh script`, `analyze-module-coupling.sh script`, `check-architecture-drift.sh script`, `generate-architecture-docs.sh script`, `generate-ctags.sh script` (+7919 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **248 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Provider` connect `Provider` to `service.rs`, `openai.rs`, `ollama.rs`, `render.rs`, `app.rs`, `error_scenarios_test.rs`, `QwenProvider`, `MockProvider`, `StreamingMockProvider`, `AnthropicProvider`, `CrabraceIntegration`, `MockProvider`, `LlamaCppProvider`, `ProviderUpdater`, `TuiEvent`, `ollama_download.rs`, `MockProvider`, `DummyProvider`, `GeminiProvider`, `AzureOpenAIProvider`, `DummyProvider`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **Why does `Functions` connect `Functions` to `Interfaces`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Why does `ToolCapability` connect `service.rs` to `SecretString`, `plan_tool_security_tests.rs`, `task.rs`, `skill.rs`, `doc_parser.rs`, `todo_write.rs`, `powershell.rs`, `ToolRegistry`, `client.rs`, `ContextStore`, `ApplyPatchTool`, `.execute`, `ToolExecutionContext`, `registry.rs`, `bash.rs`, `web_fetch.rs`, `agent.rs`, `.execute`, `notebook.rs`, `.execute`, `http.rs`, `AskUserTool`, `WebSearchTool`, `CodeExecTool`, `glob.rs`, `LsTool`, `edit.rs`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **What connects `check_plan_debug.sh script`, `analyze-module-coupling.sh script`, `check-architecture-drift.sh script` to the rest of the system?**
  _7924 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `service.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0626674912389198 - nodes in this community are weakly interconnected._
- **Should `events.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09408033826638477 - nodes in this community are weakly interconnected._
- **Should `openai.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07936507936507936 - nodes in this community are weakly interconnected._