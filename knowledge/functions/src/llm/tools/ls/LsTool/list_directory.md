---
type: Rust Method
title: list_directory
resource: src/llm/tools/ls.rs#L138-L213
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/tools/ls/LsTool/tool/execute
---

# Signature

`async fn list_directory( &self, path: &Path, input: &LsInput, output: &mut String, ) -> Result<()>`

# Calls

- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/ls/LsTool/tool/execute.md)