---
type: Rust Module
title: memory
resource: src/llm/agent/memory.rs#L1-L196
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/anyhow-result
  - external/chrono-datetime-utc
  - external/serde-deserialize-serialize
  - external/sqlx-sqlitepool
  - external/std-path-path
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [EpisodicMemory](../../../../classes/src/llm/agent/memory/EpisodicMemory.md)
- [SymbolKind](../../../../classes/src/llm/agent/memory/SymbolKind.md)
- [CodebaseIndexEntry](../../../../classes/src/llm/agent/memory/CodebaseIndexEntry.md)
- [CodebaseIndex](../../../../classes/src/llm/agent/memory/CodebaseIndex.md)
- [new](../../../../functions/src/llm/agent/memory/CodebaseIndex/new.md)
- [index_file](../../../../functions/src/llm/agent/memory/CodebaseIndex/index_file.md)
- [query_symbol](../../../../functions/src/llm/agent/memory/CodebaseIndex/query_symbol.md)
- [fts_search](../../../../functions/src/llm/agent/memory/CodebaseIndex/fts_search.md)
- [symbol_kind_str](../../../../functions/src/llm/agent/memory/symbol_kind_str.md)
- [str_to_symbol_kind](../../../../functions/src/llm/agent/memory/str_to_symbol_kind.md)
- [row_to_entry](../../../../functions/src/llm/agent/memory/row_to_entry.md)
- [extract_symbols](../../../../functions/src/llm/agent/memory/extract_symbols.md)

# Imports

- `anyhow::Result`
- `chrono::{DateTime, Utc}`
- `serde::{Deserialize, Serialize}`
- `sqlx::SqlitePool`
- `std::path::Path`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)