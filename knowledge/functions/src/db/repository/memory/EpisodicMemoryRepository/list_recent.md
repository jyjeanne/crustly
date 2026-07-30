---
type: Rust Method
title: list_recent
resource: src/db/repository/memory.rs#L41-L96
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/db/models/PlanTaskStatus/parse
  - functions/src/config/secrets/SecretString/from_str
  called_by:
  - functions/src/db/repository/memory/EpisodicMemoryRepository/inject_into_context
  - functions/src/db/repository/memory/list_recent_truncates_multibyte_summary_without_panicking
---

# Signature

`pub async fn list_recent(&self, limit: u32, max_tokens: i32) -> Result<Vec<EpisodicMemory>>`

# Calls

- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)
- [parse](../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [from_str](../../../../../../functions/src/config/secrets/SecretString/from_str.md)

# Called by

- [inject_into_context](../../../../../../functions/src/db/repository/memory/EpisodicMemoryRepository/inject_into_context.md)
- [list_recent_truncates_multibyte_summary_without_panicking](../../../../../../functions/src/db/repository/memory/list_recent_truncates_multibyte_summary_without_panicking.md)