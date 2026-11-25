---
type: Rust Method
title: with_metadata
resource: src/llm/tools/trait.rs#L234-L237
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/agent/AgentTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/report_result
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ask_user/AskUserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/BashTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/PowerShellTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/read/ReadTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/test_tool_result_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_metadata(mut self, key: String, value: String) -> Self`

# Called by

- [execute](../../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)
- [report_result](../../../../../../functions/src/llm/tools/agent/report_result.md)
- [execute](../../../../../../functions/src/llm/tools/ask_user/AskUserTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/skill/SkillTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute.md)
- [test_tool_result_success](../../../../../../functions/src/llm/tools/trait/test_tool_result_success.md)
- [execute](../../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)