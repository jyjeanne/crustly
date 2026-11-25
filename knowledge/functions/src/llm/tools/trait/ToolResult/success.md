---
type: Rust Method
title: success
resource: src/llm/tools/trait.rs#L214-L221
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/MockTool/crate-llm-tools-tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/NamedTool/crate-llm-tools-tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/BigOutputTool/crate-llm-tools-tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/WritingTool/crate-llm-tools-tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/StrictTool/crate-llm-tools-tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/agent/AgentTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/agent/report_result
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/ask_user/AskUserTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/BashTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/code_exec/CodeExecTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/context/ContextTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/EditTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/glob/GlobTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/grep/GrepTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/ls/LsTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/notebook/NotebookEditTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/powershell/PowerShellTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/read/ReadTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/MockTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/SearchMemoryTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/test_tool_result_success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/web_search/WebSearchTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/mcp/client/McpTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn success(output: String) -> Self`

# Called by

- [execute](../../../../../../functions/src/llm/agent/service/MockTool/crate-llm-tools-tool/execute.md)
- [execute](../../../../../../functions/src/llm/agent/service/NamedTool/crate-llm-tools-tool/execute.md)
- [execute](../../../../../../functions/src/llm/agent/service/BigOutputTool/crate-llm-tools-tool/execute.md)
- [execute](../../../../../../functions/src/llm/agent/service/WritingTool/crate-llm-tools-tool/execute.md)
- [execute](../../../../../../functions/src/llm/agent/service/StrictTool/crate-llm-tools-tool/execute.md)
- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [execute](../../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)
- [report_result](../../../../../../functions/src/llm/tools/agent/report_result.md)
- [execute](../../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/ask_user/AskUserTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/context/ContextTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/glob/GlobTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/ls/LsTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/registry/MockTool/tool/execute.md)
- [save_to_store](../../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)
- [execute](../../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/skill/SkillTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute.md)
- [test_tool_result_success](../../../../../../functions/src/llm/tools/trait/test_tool_result_success.md)
- [execute](../../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)
- [execute](../../../../../../functions/src/mcp/client/McpTool/tool/execute.md)