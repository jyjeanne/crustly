---
type: Rust Function
title: create_anthropic
resource: src/llm/provider/factory.rs#L522-L548
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/factory/create_provider
---

# Signature

`fn create_anthropic(config: &Config) -> Result<Arc<dyn Provider>>`

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)