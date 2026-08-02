---
type: Rust Method
title: execute
resource: src/llm/tools/web_search.rs#L129-L225
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
---

# Signature

`async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)