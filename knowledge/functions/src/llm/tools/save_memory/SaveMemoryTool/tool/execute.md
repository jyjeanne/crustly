---
type: Rust Method
title: execute
resource: src/llm/tools/save_memory.rs#L111-L153
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/tools/save_memory/memory_path
  - functions/src/llm/tools/save_memory/append_fact
  - functions/src/llm/tools/trait/ToolResult/with_metadata
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [memory_path](../../../../../../../functions/src/llm/tools/save_memory/memory_path.md)
- [append_fact](../../../../../../../functions/src/llm/tools/save_memory/append_fact.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)