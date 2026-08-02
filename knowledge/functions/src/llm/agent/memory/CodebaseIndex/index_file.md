---
type: Rust Method
title: index_file
resource: src/llm/agent/memory.rs#L61-L89
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/memory/extract_symbols
  - functions/src/llm/agent/memory/symbol_kind_str
  called_by:
  - functions/src/app/start_file_watcher
  - functions/tests/codebase_index_test/index_and_query_provider_trait
  - functions/tests/codebase_index_test/index_file_twice_no_duplicate
  - functions/tests/codebase_index_test/fts_search_finds_symbol_by_partial_name
  - functions/tests/codebase_index_test/index_nonexistent_file_returns_error
---

# Signature

`pub async fn index_file(&self, path: &Path) -> Result<()>`

# Calls

- [extract_symbols](../../../../../../functions/src/llm/agent/memory/extract_symbols.md)
- [symbol_kind_str](../../../../../../functions/src/llm/agent/memory/symbol_kind_str.md)

# Called by

- [start_file_watcher](../../../../../../functions/src/app/start_file_watcher.md)
- [index_and_query_provider_trait](../../../../../../functions/tests/codebase_index_test/index_and_query_provider_trait.md)
- [index_file_twice_no_duplicate](../../../../../../functions/tests/codebase_index_test/index_file_twice_no_duplicate.md)
- [fts_search_finds_symbol_by_partial_name](../../../../../../functions/tests/codebase_index_test/fts_search_finds_symbol_by_partial_name.md)
- [index_nonexistent_file_returns_error](../../../../../../functions/tests/codebase_index_test/index_nonexistent_file_returns_error.md)