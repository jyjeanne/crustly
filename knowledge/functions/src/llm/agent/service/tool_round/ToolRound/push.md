---
type: Rust Method
title: push
resource: src/llm/agent/service/tool_round.rs#L38-L41
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/memory_status_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/summarize_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/append_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/register
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/refuse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/pair_mmproj_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/llm_response_to_stream_events
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/parse_patch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/apply_hunks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/code_exec/CodeExecTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/context/ContextTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_xml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/resolve_existing_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/append_fact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/send_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/notify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/unmatched_memories_never_take_budget_when_others_compete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/spawn_worker
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/tasks_in_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/record_tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_topological_sort_with_dependencies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_topological_sort_circular_dependency
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_validate_dependencies_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_validate_dependencies_circular
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_complex_dependency_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/memory_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/reload_memory_rows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_memory_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/description
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/highlight_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_perf_footer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_thinking_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_message_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_processing_indicator
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_plan_task_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_plan_document
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_capabilities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_preview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_input_detailed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_input_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_model_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/StreamingMockProvider/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_basic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/test_streaming_multiple_chunks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn push(&mut self, (block, record): (ContentBlock, ToolCallRecord))`

# Called by

- [memory_status_lines](../../../../../../../functions/src/cli/memory_status_lines.md)
- [llama_cpp_doctor_findings](../../../../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [summarize_tool_calls](../../../../../../../functions/src/llm/agent/project_memory/summarize_tool_calls.md)
- [append_progress](../../../../../../../functions/src/llm/agent/project_memory/append_progress.md)
- [register](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/register.md)
- [refuse](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/refuse.md)
- [execute_tool_round](../../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [merge_split_gguf_groups](../../../../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups.md)
- [pair_mmproj_files](../../../../../../../functions/src/llm/provider/llama_cpp_models/pair_mmproj_files.md)
- [llm_response_to_stream_events](../../../../../../../functions/src/llm/provider/qwen/llm_response_to_stream_events.md)
- [parse_patch](../../../../../../../functions/src/llm/tools/apply_patch/parse_patch.md)
- [apply_hunks](../../../../../../../functions/src/llm/tools/apply_patch/apply_hunks.md)
- [execute](../../../../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/execute.md)
- [execute](../../../../../../../functions/src/llm/tools/context/ContextTool/tool/execute.md)
- [parse_xml](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_xml.md)
- [execute](../../../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [execute](../../../../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [resolve_existing_prefix](../../../../../../../functions/src/llm/tools/sandbox/resolve_existing_prefix.md)
- [append_fact](../../../../../../../functions/src/llm/tools/save_memory/append_fact.md)
- [execute](../../../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)
- [send_request](../../../../../../../functions/src/mcp/client/MCPClient/send_request.md)
- [consolidate](../../../../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [notify](../../../../../../../functions/src/memory/MemoryStore/notify.md)
- [unmatched_memories_never_take_budget_when_others_compete](../../../../../../../functions/src/memory/recall/unmatched_memories_never_take_budget_when_others_compete.md)
- [spawn_worker](../../../../../../../functions/src/memory/semantic/spawn_worker.md)
- [import](../../../../../../../functions/src/memory/transfer/import.md)
- [tasks_in_order](../../../../../../../functions/src/plan/PlanDocument/tasks_in_order.md)
- [record_tool_call](../../../../../../../functions/src/plan/PlanTask/record_tool_call.md)
- [test_topological_sort_with_dependencies](../../../../../../../functions/src/plan/plan_tests/test_topological_sort_with_dependencies.md)
- [test_topological_sort_circular_dependency](../../../../../../../functions/src/plan/plan_tests/test_topological_sort_circular_dependency.md)
- [test_validate_dependencies_success](../../../../../../../functions/src/plan/plan_tests/test_validate_dependencies_success.md)
- [test_validate_dependencies_invalid_reference](../../../../../../../functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference.md)
- [test_validate_dependencies_circular](../../../../../../../functions/src/plan/plan_tests/test_validate_dependencies_circular.md)
- [test_complex_dependency_chain](../../../../../../../functions/src/plan/plan_tests/test_complex_dependency_chain.md)
- [memory_command](../../../../../../../functions/src/tui/app/App/memory_command.md)
- [reload_memory_rows](../../../../../../../functions/src/tui/app/App/reload_memory_rows.md)
- [handle_memory_key](../../../../../../../functions/src/tui/app/App/handle_memory_key.md)
- [description](../../../../../../../functions/src/tui/error/ErrorInfo/description.md)
- [highlight_code](../../../../../../../functions/src/tui/highlight/highlight_code.md)
- [render_header](../../../../../../../functions/src/tui/render/render_header.md)
- [render_perf_footer](../../../../../../../functions/src/tui/render/render_perf_footer.md)
- [render_thinking_block](../../../../../../../functions/src/tui/render/render_thinking_block.md)
- [render_message_lines](../../../../../../../functions/src/tui/render/render_message_lines.md)
- [render_processing_indicator](../../../../../../../functions/src/tui/render/render_processing_indicator.md)
- [render_plan_task_lines](../../../../../../../functions/src/tui/render/render_plan_task_lines.md)
- [render_plan_document](../../../../../../../functions/src/tui/render/render_plan_document.md)
- [render_approval_capabilities](../../../../../../../functions/src/tui/render/render_approval_capabilities.md)
- [render_approval_preview](../../../../../../../functions/src/tui/render/render_approval_preview.md)
- [render_approval_input_detailed](../../../../../../../functions/src/tui/render/render_approval_input_detailed.md)
- [render_approval_input_summary](../../../../../../../functions/src/tui/render/render_approval_input_summary.md)
- [llama_cpp_model_lines](../../../../../../../functions/src/tui/render/llama_cpp_model_lines.md)
- [a_secret_never_reaches_the_database_file_or_its_wal](../../../../../../../functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal.md)
- [new](../../../../../../../functions/tests/streaming_test/StreamingMockProvider/new.md)
- [test_streaming_basic](../../../../../../../functions/tests/streaming_test/test_streaming_basic.md)
- [test_streaming_multiple_chunks](../../../../../../../functions/tests/streaming_test/test_streaming_multiple_chunks.md)