---
type: Rust Method
title: search
resource: src/memory/mod.rs#L603-L630
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/recall/fts_match_expr
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/fts_search
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/SearchMemoryTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn search( &self, project: &ProjectIdentity, query: &str, scope: SearchScope, category: Option<MemoryCategory>, limit: usize, ) -> Result<Vec<Memory>>`

# Calls

- [fts_match_expr](../../../../functions/src/memory/recall/fts_match_expr.md)
- [fts_search](../../../../functions/src/db/repository/memory/MemoryRepository/fts_search.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [execute](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/execute.md)
- [extract](../../../../functions/src/memory/extraction/MemoryStore/extract.md)