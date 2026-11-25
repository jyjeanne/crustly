---
type: Rust Method
title: search_file
resource: src/llm/tools/grep.rs#L238-L312
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/grep/GrepTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn search_file( &self, path: &Path, regex: &regex::Regex, input: &GrepInput, matches: &mut Vec<String>, total_matches: &mut usize, ) -> Result<()>`

# Calls

- [skip](../../../../../../functions/src/plan/PlanTask/skip.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)