---
type: Rust Function
title: summarise_turns
resource: src/llm/agent/compaction.rs#L133-L157
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/agent/compaction/compact
  - functions/src/llm/agent/compaction/summarise_turns_truncates_multibyte_text_without_panicking
---

# Signature

`fn summarise_turns(messages: &[crate::llm::provider::types::Message]) -> String`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [compact](../../../../../functions/src/llm/agent/compaction/compact.md)
- [summarise_turns_truncates_multibyte_text_without_panicking](../../../../../functions/src/llm/agent/compaction/summarise_turns_truncates_multibyte_text_without_panicking.md)