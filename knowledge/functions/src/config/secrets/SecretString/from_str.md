---
type: Rust Method
title: from_str
resource: src/config/secrets.rs#L33-L37
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/cli/cmd_keyring
  - functions/src/config/Config/merge_from_file
  - functions/src/config/test_config_from_toml
  - functions/src/config/test_config_save_and_load
  - functions/src/config/secrets/SecretString/from-str/from
  - functions/src/config/secrets/test_secret_string_creation
  - functions/src/config/secrets/test_secret_string_debug
  - functions/src/config/secrets/test_secret_string_display
  - functions/src/config/secrets/test_provider_secrets_with_keys
  - functions/src/config/secrets/test_secret_string_serialize
  - functions/src/db/repository/memory/EpisodicMemoryRepository/list_recent
  - functions/src/db/repository/plan/PlanRepository/plan_from_db
  - functions/src/db/repository/plan/PlanRepository/task_from_db
  - functions/src/llm/agent/service/apply_streamed_tool_input
  - functions/src/llm/provider/openai/OpenAIProvider/from_openai_response
  - functions/src/llm/provider/openai/OpenAIProvider/provider/stream
  - functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
  - functions/src/llm/provider/tool_call_recovery/parse_tool_call_object
  - functions/src/llm/tools/context/ContextStore/load
  - functions/src/llm/tools/doc_parser/DocParserTool/extract_text_from_docx_xml
  - functions/src/llm/tools/doc_parser/DocParserTool/extract_metadata_from_core_xml
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_xml
  - functions/src/llm/tools/http/HttpClientTool/tool/execute
  - functions/src/llm/tools/notebook/NotebookEditTool/tool/execute
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
  - functions/src/llm/tools/task/TaskStore/load
  - functions/src/llm/tools/todo_write/TodoStore/load
  - functions/src/mcp/client/match_response_line
  - functions/src/services/message/test_update_message_metrics_with_perf_data
  - functions/src/services/plan/PlanService/import_from_json
  - functions/src/services/plan/test_service_export_to_json
  - functions/src/tui/app/DisplayMessage/from-message/from
---

# Signature

`pub fn from_str(value: &str) -> Self`

# Called by

- [cmd_keyring](../../../../../functions/src/cli/cmd_keyring.md)
- [merge_from_file](../../../../../functions/src/config/Config/merge_from_file.md)
- [test_config_from_toml](../../../../../functions/src/config/test_config_from_toml.md)
- [test_config_save_and_load](../../../../../functions/src/config/test_config_save_and_load.md)
- [from](../../../../../functions/src/config/secrets/SecretString/from-str/from.md)
- [test_secret_string_creation](../../../../../functions/src/config/secrets/test_secret_string_creation.md)
- [test_secret_string_debug](../../../../../functions/src/config/secrets/test_secret_string_debug.md)
- [test_secret_string_display](../../../../../functions/src/config/secrets/test_secret_string_display.md)
- [test_provider_secrets_with_keys](../../../../../functions/src/config/secrets/test_provider_secrets_with_keys.md)
- [test_secret_string_serialize](../../../../../functions/src/config/secrets/test_secret_string_serialize.md)
- [list_recent](../../../../../functions/src/db/repository/memory/EpisodicMemoryRepository/list_recent.md)
- [plan_from_db](../../../../../functions/src/db/repository/plan/PlanRepository/plan_from_db.md)
- [task_from_db](../../../../../functions/src/db/repository/plan/PlanRepository/task_from_db.md)
- [apply_streamed_tool_input](../../../../../functions/src/llm/agent/service/apply_streamed_tool_input.md)
- [from_openai_response](../../../../../functions/src/llm/provider/openai/OpenAIProvider/from_openai_response.md)
- [stream](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [parse_fallback_tool_calls](../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)
- [from_qwen_response](../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [parse_tool_call_object](../../../../../functions/src/llm/provider/tool_call_recovery/parse_tool_call_object.md)
- [load](../../../../../functions/src/llm/tools/context/ContextStore/load.md)
- [extract_text_from_docx_xml](../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_text_from_docx_xml.md)
- [extract_metadata_from_core_xml](../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_metadata_from_core_xml.md)
- [parse_xml](../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_xml.md)
- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [load](../../../../../functions/src/llm/tools/task/TaskStore/load.md)
- [load](../../../../../functions/src/llm/tools/todo_write/TodoStore/load.md)
- [match_response_line](../../../../../functions/src/mcp/client/match_response_line.md)
- [test_update_message_metrics_with_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_with_perf_data.md)
- [import_from_json](../../../../../functions/src/services/plan/PlanService/import_from_json.md)
- [test_service_export_to_json](../../../../../functions/src/services/plan/test_service_export_to_json.md)
- [from](../../../../../functions/src/tui/app/DisplayMessage/from-message/from.md)