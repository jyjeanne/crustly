---
type: Rust Function
title: try_create_openai
resource: src/llm/provider/factory.rs#L511-L534
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/configure_openai
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/factory/create_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/create_named_provider
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_create_openai(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [configure_openai](../../../../../functions/src/llm/provider/factory/configure_openai.md)
- [local](../../../../../functions/src/llm/provider/openai/OpenAIProvider/local.md)
- [new](../../../../../functions/src/llm/provider/openai/OpenAIProvider/new.md)

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)
- [create_named_provider](../../../../../functions/src/llm/provider/factory/create_named_provider.md)