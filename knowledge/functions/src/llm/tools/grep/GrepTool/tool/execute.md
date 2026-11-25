---
type: Rust Method
title: execute
resource: src/llm/tools/grep.rs#L141-L234
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/GrepTool/search_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/collect_searchable_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/grep/test_grep_accepts_glob_alias_for_file_pattern
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/grep/test_pattern_is_regex_by_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/grep/test_regex_false_still_searches_literally
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/grep/test_search_respects_gitignore
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [build](../../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
- [search_file](../../../../../../../functions/src/llm/tools/grep/GrepTool/search_file.md)
- [collect_searchable_files](../../../../../../../functions/src/llm/tools/grep/collect_searchable_files.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [test_grep_accepts_glob_alias_for_file_pattern](../../../../../../../functions/src/llm/tools/grep/test_grep_accepts_glob_alias_for_file_pattern.md)
- [test_pattern_is_regex_by_default](../../../../../../../functions/src/llm/tools/grep/test_pattern_is_regex_by_default.md)
- [test_regex_false_still_searches_literally](../../../../../../../functions/src/llm/tools/grep/test_regex_false_still_searches_literally.md)
- [test_search_respects_gitignore](../../../../../../../functions/src/llm/tools/grep/test_search_respects_gitignore.md)