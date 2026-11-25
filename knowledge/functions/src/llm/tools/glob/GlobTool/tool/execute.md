---
type: Rust Method
title: execute
resource: src/llm/tools/glob.rs#L91-L191
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/glob/test_glob_matches_recursive_pattern
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/glob/test_glob_respects_gitignore
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/glob/test_glob_no_matches
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/glob/test_glob_respects_limit
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [exists](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [build](../../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [test_glob_matches_recursive_pattern](../../../../../../../functions/src/llm/tools/glob/test_glob_matches_recursive_pattern.md)
- [test_glob_respects_gitignore](../../../../../../../functions/src/llm/tools/glob/test_glob_respects_gitignore.md)
- [test_glob_no_matches](../../../../../../../functions/src/llm/tools/glob/test_glob_no_matches.md)
- [test_glob_respects_limit](../../../../../../../functions/src/llm/tools/glob/test_glob_respects_limit.md)