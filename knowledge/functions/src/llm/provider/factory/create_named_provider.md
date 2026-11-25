---
type: Rust Function
title: create_named_provider
resource: src/llm/provider/factory.rs#L202-L220
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/create_anthropic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/try_create_openai
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/try_create_gemini
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/try_create_azure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/try_create_qwen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/named_providers_must_be_known_and_configured
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn create_named_provider(config: &Config, name: &str) -> Result<Arc<dyn Provider>>`

# Calls

- [create_anthropic](../../../../../functions/src/llm/provider/factory/create_anthropic.md)
- [try_create_openai](../../../../../functions/src/llm/provider/factory/try_create_openai.md)
- [try_create_gemini](../../../../../functions/src/llm/provider/factory/try_create_gemini.md)
- [try_create_azure](../../../../../functions/src/llm/provider/factory/try_create_azure.md)
- [try_create_qwen](../../../../../functions/src/llm/provider/factory/try_create_qwen.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [configure_agent](../../../../../functions/src/cli/configure_agent.md)
- [named_providers_must_be_known_and_configured](../../../../../functions/src/llm/provider/factory/named_providers_must_be_known_and_configured.md)