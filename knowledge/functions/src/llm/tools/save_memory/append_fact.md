---
type: Rust Function
title: append_fact
resource: src/llm/tools/save_memory.rs#L44-L65
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/tui/events/EventHandler/next
  called_by:
  - functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
  - functions/src/llm/tools/save_memory/append_fact_adds_header_to_a_file_that_lacks_one
---

# Signature

`fn append_fact(existing: &str, fact: &str) -> (String, bool)`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [append_fact_adds_header_to_a_file_that_lacks_one](../../../../../functions/src/llm/tools/save_memory/append_fact_adds_header_to_a_file_that_lacks_one.md)