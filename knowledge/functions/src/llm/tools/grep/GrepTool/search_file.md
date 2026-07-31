---
type: Rust Method
title: search_file
resource: src/llm/tools/grep.rs#L238-L312
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/plan/PlanTask/skip
  called_by:
  - functions/src/llm/tools/grep/GrepTool/tool/execute
---

# Signature

`async fn search_file( &self, path: &Path, regex: &regex::Regex, input: &GrepInput, matches: &mut Vec<String>, total_matches: &mut usize, ) -> Result<()>`

# Calls

- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)
- [skip](../../../../../../functions/src/plan/PlanTask/skip.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)