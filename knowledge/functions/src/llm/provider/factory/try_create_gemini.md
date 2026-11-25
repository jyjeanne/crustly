---
type: Rust Function
title: try_create_gemini
resource: src/llm/provider/factory.rs#L260-L284
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/GeminiProvider/with_base_url
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/GeminiProvider/new
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

`fn try_create_gemini(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [with_base_url](../../../../../functions/src/llm/provider/gemini/GeminiProvider/with_base_url.md)
- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)
- [create_named_provider](../../../../../functions/src/llm/provider/factory/create_named_provider.md)