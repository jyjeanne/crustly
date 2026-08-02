---
type: Rust Function
title: try_create_gemini
resource: src/llm/provider/factory.rs#L216-L242
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/factory/create_provider
---

# Signature

`fn try_create_gemini(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)