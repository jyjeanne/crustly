---
type: Rust Function
title: id_for_root
resource: src/memory/project.rs#L28-L30
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/sha256_hex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/normalize_root
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/transfer/import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/repo
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_anchors_test/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/store_and_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/store_and_real_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/project_at
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn id_for_root(root: &Path) -> ProjectId`

# Calls

- [sha256_hex](../../../../functions/src/memory/project/sha256_hex.md)
- [normalize_root](../../../../functions/src/memory/project/normalize_root.md)

# Called by

- [import](../../../../functions/src/memory/transfer/import.md)
- [repo](../../../../functions/tests/memory_acceptance_test/repo.md)
- [setup](../../../../functions/tests/memory_anchors_test/setup.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [setup](../../../../functions/tests/memory_code_knowledge_test/setup.md)
- [project](../../../../functions/tests/memory_consolidate_test/project.md)
- [project](../../../../functions/tests/memory_extraction_test/project.md)
- [store_and_project](../../../../functions/tests/memory_paths_test/store_and_project.md)
- [store_and_real_project](../../../../functions/tests/memory_paths_test/store_and_real_project.md)
- [project](../../../../functions/tests/memory_semantic_test/project.md)
- [project_at](../../../../functions/tests/memory_transfer_test/project_at.md)