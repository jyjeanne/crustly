---
type: Rust Function
title: index_nonexistent_file_returns_error
resource: tests/codebase_index_test.rs#L105-L114
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/tests/codebase_index_test/create_pool_with_schema
  - functions/src/llm/agent/memory/CodebaseIndex/index_file
---

# Signature

`async fn index_nonexistent_file_returns_error()`

# Calls

- [create_pool_with_schema](../../../functions/tests/codebase_index_test/create_pool_with_schema.md)
- [index_file](../../../functions/src/llm/agent/memory/CodebaseIndex/index_file.md)