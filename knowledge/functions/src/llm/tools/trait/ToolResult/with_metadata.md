---
type: Rust Method
title: with_metadata
resource: src/llm/tools/trait.rs#L158-L161
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/agent/AgentTool/tool/execute
  - functions/src/llm/tools/ask_user/AskUserTool/tool/execute
  - functions/src/llm/tools/bash/BashTool/tool/execute
  - functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
  - functions/src/llm/tools/powershell/PowerShellTool/tool/execute
  - functions/src/llm/tools/read/ReadTool/tool/execute
  - functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
  - functions/src/llm/tools/skill/SkillTool/tool/execute
  - functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute
  - functions/src/llm/tools/trait/test_tool_result_success
  - functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
  - functions/src/llm/tools/write/WriteTool/tool/execute
---

# Signature

`pub fn with_metadata(mut self, key: String, value: String) -> Self`

# Called by

- [execute](../../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)
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