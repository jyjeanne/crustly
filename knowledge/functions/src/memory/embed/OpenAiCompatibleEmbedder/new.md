---
type: Rust Method
title: new
resource: src/memory/embed.rs#L255-L264
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/embed/check_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/client
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/embed/openai_orders_by_index_and_sends_the_key
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embedder_from_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(settings: EmbedSettings) -> Result<Self, EmbedError>`

# Calls

- [check_settings](../../../../../functions/src/memory/embed/check_settings.md)
- [client](../../../../../functions/src/memory/embed/client.md)

# Called by

- [openai_orders_by_index_and_sends_the_key](../../../../../functions/src/memory/embed/openai_orders_by_index_and_sends_the_key.md)
- [embedder_from_config](../../../../../functions/src/memory/embedder_from_config.md)