---
type: Rust Function
title: create_anthropic
resource: src/llm/provider/factory.rs#L546-L571
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/new
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

`fn create_anthropic(config: &Config) -> Result<Arc<dyn Provider>>`

# Calls

- [context](../../../../../functions/src/services/ServiceManager/context.md)
- [new](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/new.md)

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)
- [create_named_provider](../../../../../functions/src/llm/provider/factory/create_named_provider.md)