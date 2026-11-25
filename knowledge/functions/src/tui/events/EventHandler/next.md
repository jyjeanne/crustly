---
type: Rust Method
title: next
resource: src/tui/events.rs#L253-L255
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/bundled_sqlite_supports_fts5_secure_delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/call_target
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_work_is_remembered_across_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/from_gemini_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/parse_gemini_sse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_rocm_smi_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_vulkaninfo_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/pull_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/from_openai_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/is_read_only_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/forget_memory/resolve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/strip_verbatim_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/BashCommandAllowlist/permissionpolicy/evaluate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/find_active_shell_operator
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/append_fact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/parse_skill_frontmatter_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/Prefixes/for_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/normalize_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/get_most_recent_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/get_most_recent_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/first_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/next_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/forget_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_basic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_single_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_multiple_chunks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_token_counting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_stop_reason
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_error_handling
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_empty_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_content_accumulation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn next(&mut self) -> Option<TuiEvent>`

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [bundled_sqlite_supports_fts5_secure_delete](../../../../../functions/src/db/bundled_sqlite_supports_fts5_secure_delete.md)
- [call_target](../../../../../functions/src/llm/agent/project_memory/call_target.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [tool_work_is_remembered_across_requests](../../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
- [execute_tool_round](../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [from_gemini_response](../../../../../functions/src/llm/provider/gemini/GeminiProvider/from_gemini_response.md)
- [parse_gemini_sse](../../../../../functions/src/llm/provider/gemini/parse_gemini_sse.md)
- [stream](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)
- [parse_nvidia_smi_csv](../../../../../functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv.md)
- [parse_rocm_smi_json](../../../../../functions/src/llm/provider/hardware_detect/parse_rocm_smi_json.md)
- [parse_vulkaninfo_summary](../../../../../functions/src/llm/provider/hardware_detect/parse_vulkaninfo_summary.md)
- [parse_hf_shorthand](../../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [streamed_tool_call_reaches_caller](../../../../../functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller.md)
- [text_arrives_before_the_response_is_complete](../../../../../functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete.md)
- [a_reply_cut_off_before_the_done_chunk_is_an_error](../../../../../functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error.md)
- [pull_model](../../../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [from_openai_response](../../../../../functions/src/llm/provider/openai/OpenAIProvider/from_openai_response.md)
- [stream](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [text_arrives_before_the_response_is_complete](../../../../../functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete.md)
- [from_qwen_response](../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [stream](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [is_read_only_command](../../../../../functions/src/llm/tools/bash/is_read_only_command.md)
- [resolve](../../../../../functions/src/llm/tools/forget_memory/resolve.md)
- [strip_verbatim_prefix](../../../../../functions/src/llm/tools/sandbox/strip_verbatim_prefix.md)
- [evaluate](../../../../../functions/src/llm/tools/sandbox/BashCommandAllowlist/permissionpolicy/evaluate.md)
- [find_active_shell_operator](../../../../../functions/src/llm/tools/sandbox/find_active_shell_operator.md)
- [append_fact](../../../../../functions/src/llm/tools/save_memory/append_fact.md)
- [parse_skill_frontmatter_value](../../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)
- [for_model](../../../../../functions/src/memory/embed/Prefixes/for_model.md)
- [normalize_root](../../../../../functions/src/memory/project/normalize_root.md)
- [get_most_recent_plan](../../../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)
- [get_most_recent_session](../../../../../functions/src/services/session/SessionService/get_most_recent_session.md)
- [first_line](../../../../../functions/src/tui/app/first_line.md)
- [next_event](../../../../../functions/src/tui/app/App/next_event.md)
- [forget_command](../../../../../functions/src/tui/app/App/forget_command.md)
- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [test_streaming_basic](../../../../../functions/tests/streaming_test/test_streaming_basic.md)
- [test_streaming_single_chunk](../../../../../functions/tests/streaming_test/test_streaming_single_chunk.md)
- [test_streaming_multiple_chunks](../../../../../functions/tests/streaming_test/test_streaming_multiple_chunks.md)
- [test_streaming_token_counting](../../../../../functions/tests/streaming_test/test_streaming_token_counting.md)
- [test_streaming_stop_reason](../../../../../functions/tests/streaming_test/test_streaming_stop_reason.md)
- [test_streaming_error_handling](../../../../../functions/tests/streaming_test/test_streaming_error_handling.md)
- [test_streaming_empty_response](../../../../../functions/tests/streaming_test/test_streaming_empty_response.md)
- [test_streaming_content_accumulation](../../../../../functions/tests/streaming_test/test_streaming_content_accumulation.md)