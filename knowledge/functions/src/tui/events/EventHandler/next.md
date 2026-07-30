---
type: Rust Method
title: next
resource: src/tui/events.rs#L211-L213
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/cli/cmd_ollama
  - functions/src/llm/agent/service/drain_stream_to_response
  - functions/src/llm/provider/gemini/GeminiProvider/from_gemini_response
  - functions/src/llm/provider/gemini/parse_gemini_sse
  - functions/src/llm/provider/gemini/GeminiProvider/provider/stream
  - functions/src/llm/provider/ollama/OllamaProvider/provider/stream
  - functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller
  - functions/src/llm/provider/ollama_models/pull_model
  - functions/src/llm/provider/openai/OpenAIProvider/from_openai_response
  - functions/src/llm/provider/openai/OpenAIProvider/provider/stream
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
  - functions/src/llm/provider/qwen/QwenProvider/provider/stream
  - functions/src/llm/tools/bash/is_read_only_command
  - functions/src/llm/tools/sandbox/strip_verbatim_prefix
  - functions/src/llm/tools/sandbox/BashCommandAllowlist/permissionpolicy/evaluate
  - functions/src/llm/tools/sandbox/find_active_shell_operator
  - functions/src/llm/tools/save_memory/append_fact
  - functions/src/llm/tools/skill/parse_skill_frontmatter_value
  - functions/src/services/plan/PlanService/get_most_recent_plan
  - functions/src/services/session/SessionService/get_most_recent_session
  - functions/src/tui/app/App/next_event
  - functions/src/tui/app/App/try_handle_slash_command
  - functions/tests/streaming_test/test_streaming_basic
  - functions/tests/streaming_test/test_streaming_single_chunk
  - functions/tests/streaming_test/test_streaming_multiple_chunks
  - functions/tests/streaming_test/test_streaming_token_counting
  - functions/tests/streaming_test/test_streaming_stop_reason
  - functions/tests/streaming_test/test_streaming_error_handling
  - functions/tests/streaming_test/test_streaming_empty_response
  - functions/tests/streaming_test/test_streaming_content_accumulation
---

# Signature

`pub async fn next(&mut self) -> Option<TuiEvent>`

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)
- [from_gemini_response](../../../../../functions/src/llm/provider/gemini/GeminiProvider/from_gemini_response.md)
- [parse_gemini_sse](../../../../../functions/src/llm/provider/gemini/parse_gemini_sse.md)
- [stream](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)
- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [streamed_tool_call_reaches_caller](../../../../../functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller.md)
- [pull_model](../../../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [from_openai_response](../../../../../functions/src/llm/provider/openai/OpenAIProvider/from_openai_response.md)
- [stream](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [from_qwen_response](../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [stream](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [is_read_only_command](../../../../../functions/src/llm/tools/bash/is_read_only_command.md)
- [strip_verbatim_prefix](../../../../../functions/src/llm/tools/sandbox/strip_verbatim_prefix.md)
- [evaluate](../../../../../functions/src/llm/tools/sandbox/BashCommandAllowlist/permissionpolicy/evaluate.md)
- [find_active_shell_operator](../../../../../functions/src/llm/tools/sandbox/find_active_shell_operator.md)
- [append_fact](../../../../../functions/src/llm/tools/save_memory/append_fact.md)
- [parse_skill_frontmatter_value](../../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)
- [get_most_recent_plan](../../../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)
- [get_most_recent_session](../../../../../functions/src/services/session/SessionService/get_most_recent_session.md)
- [next_event](../../../../../functions/src/tui/app/App/next_event.md)
- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [test_streaming_basic](../../../../../functions/tests/streaming_test/test_streaming_basic.md)
- [test_streaming_single_chunk](../../../../../functions/tests/streaming_test/test_streaming_single_chunk.md)
- [test_streaming_multiple_chunks](../../../../../functions/tests/streaming_test/test_streaming_multiple_chunks.md)
- [test_streaming_token_counting](../../../../../functions/tests/streaming_test/test_streaming_token_counting.md)
- [test_streaming_stop_reason](../../../../../functions/tests/streaming_test/test_streaming_stop_reason.md)
- [test_streaming_error_handling](../../../../../functions/tests/streaming_test/test_streaming_error_handling.md)
- [test_streaming_empty_response](../../../../../functions/tests/streaming_test/test_streaming_empty_response.md)
- [test_streaming_content_accumulation](../../../../../functions/tests/streaming_test/test_streaming_content_accumulation.md)