---
type: Rust Function
title: row_to_entry
resource: src/llm/agent/memory.rs#L148-L158
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  - functions/src/llm/agent/memory/str_to_symbol_kind
---

# Signature

`fn row_to_entry(row: (String, String, String, String, i64, i64)) -> CodebaseIndexEntry`

# Calls

- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [str_to_symbol_kind](../../../../../functions/src/llm/agent/memory/str_to_symbol_kind.md)