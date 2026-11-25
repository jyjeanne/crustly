---
type: Rust Function
title: open
resource: tests/memory_acceptance_test.rs#L17-L22
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_logs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/read_gguf_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_docx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/read/ReadTool/read_with_buffer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/a_preference_survives_a_restart_and_is_recalled_in_another_repository
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/a_project_fact_is_never_recalled_in_another_repository
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/an_updated_decision_supersedes_the_old_one_and_keeps_its_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn open(path: &Path) -> (Database, MemoryStore)`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)

# Called by

- [cmd_logs](../../../functions/src/cli/cmd_logs.md)
- [read_gguf_metadata](../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)
- [download_model](../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [parse_docx](../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_docx.md)
- [read_with_buffer](../../../functions/src/llm/tools/read/ReadTool/read_with_buffer.md)
- [a_preference_survives_a_restart_and_is_recalled_in_another_repository](../../../functions/tests/memory_acceptance_test/a_preference_survives_a_restart_and_is_recalled_in_another_repository.md)
- [a_project_fact_is_never_recalled_in_another_repository](../../../functions/tests/memory_acceptance_test/a_project_fact_is_never_recalled_in_another_repository.md)
- [an_updated_decision_supersedes_the_old_one_and_keeps_its_history](../../../functions/tests/memory_acceptance_test/an_updated_decision_supersedes_the_old_one_and_keeps_its_history.md)
- [a_secret_never_reaches_the_database_file_or_its_wal](../../../functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal.md)