---
type: Rust Method
title: fts_search
resource: src/llm/agent/memory.rs#L105-L119
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/tests/codebase_index_test/fts_search_finds_symbol_by_partial_name
---

# Signature

`pub async fn fts_search(&self, query: &str) -> Result<Vec<CodebaseIndexEntry>>`

# Called by

- [fts_search_finds_symbol_by_partial_name](../../../../../../functions/tests/codebase_index_test/fts_search_finds_symbol_by_partial_name.md)