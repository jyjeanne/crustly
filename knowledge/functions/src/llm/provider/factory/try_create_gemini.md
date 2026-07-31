---
type: Rust Function
title: try_create_gemini
resource: src/llm/provider/factory.rs#L211-L237
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/factory/create_provider
---

# Signature

`fn try_create_gemini(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)