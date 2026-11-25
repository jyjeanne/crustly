---
type: Rust Function
title: try_create_llama_cpp
resource: src/llm/provider/factory.rs#L390-L405
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/LlamaCppProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_create_llama_cpp(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [new](../../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/new.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)