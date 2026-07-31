---
type: Rust Method
title: skip
resource: src/plan/mod.rs#L697-L702
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls
  - functions/src/llm/tools/grep/GrepTool/search_file
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
  - functions/src/llm/tools/sandbox/strip_verbatim_prefix
  - functions/src/plan/plan_tests/test_task_skip
  - functions/src/tui/render/render_file_picker
---

# Signature

`pub fn skip(&mut self, reason: Option<String>)`

# Called by

- [parse_native_qwen_tool_calls](../../../../functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls.md)
- [search_file](../../../../functions/src/llm/tools/grep/GrepTool/search_file.md)
- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [strip_verbatim_prefix](../../../../functions/src/llm/tools/sandbox/strip_verbatim_prefix.md)
- [test_task_skip](../../../../functions/src/plan/plan_tests/test_task_skip.md)
- [render_file_picker](../../../../functions/src/tui/render/render_file_picker.md)