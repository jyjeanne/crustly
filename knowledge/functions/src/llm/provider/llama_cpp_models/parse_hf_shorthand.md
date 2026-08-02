---
type: Rust Function
title: parse_hf_shorthand
resource: src/llm/provider/llama_cpp_models.rs#L118-L128
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/llama_cpp_models/resolve_download_source
---

# Signature

`fn parse_hf_shorthand(source: &str) -> Option<(&str, &str, &str)>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [resolve_download_source](../../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source.md)