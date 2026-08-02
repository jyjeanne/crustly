---
type: Rust Function
title: index_file_twice_no_duplicate
resource: tests/codebase_index_test.rs#L60-L77
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/tests/codebase_index_test/create_pool_with_schema
  - functions/src/llm/agent/memory/CodebaseIndex/index_file
  - functions/src/llm/agent/memory/CodebaseIndex/query_symbol
---

# Signature

`async fn index_file_twice_no_duplicate()`

# Calls

- [create_pool_with_schema](../../../functions/tests/codebase_index_test/create_pool_with_schema.md)
- [index_file](../../../functions/src/llm/agent/memory/CodebaseIndex/index_file.md)
- [query_symbol](../../../functions/src/llm/agent/memory/CodebaseIndex/query_symbol.md)