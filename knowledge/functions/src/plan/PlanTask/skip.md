---
type: Rust Method
title: skip
resource: src/plan/mod.rs#L697-L702
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/GrepTool/search_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/strip_verbatim_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/remove_bullets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_task_skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/fit_scroll
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_file_picker
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn skip(&mut self, reason: Option<String>)`

# Called by

- [parse_native_qwen_tool_calls](../../../../functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls.md)
- [search_file](../../../../functions/src/llm/tools/grep/GrepTool/search_file.md)
- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [strip_verbatim_prefix](../../../../functions/src/llm/tools/sandbox/strip_verbatim_prefix.md)
- [consolidate](../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [remove_bullets](../../../../functions/src/memory/transfer/remove_bullets.md)
- [test_task_skip](../../../../functions/src/plan/plan_tests/test_task_skip.md)
- [fit_scroll](../../../../functions/src/tui/render/fit_scroll.md)
- [render_memory](../../../../functions/src/tui/render/render_memory.md)
- [render_file_picker](../../../../functions/src/tui/render/render_file_picker.md)