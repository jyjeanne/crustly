---
type: Rust Function
title: try_create_openai
resource: src/llm/provider/factory.rs#L438-L463
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/factory/configure_openai
  called_by:
  - functions/src/llm/provider/factory/create_provider
---

# Signature

`fn try_create_openai(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [configure_openai](../../../../../functions/src/llm/provider/factory/configure_openai.md)

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)