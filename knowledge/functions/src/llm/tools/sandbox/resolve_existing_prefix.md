---
type: Rust Function
title: resolve_existing_prefix
resource: src/llm/tools/sandbox.rs#L455-L480
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/normalize_path
  called_by:
  - functions/src/llm/tools/sandbox/PathBoundaryRule/check
---

# Signature

`fn resolve_existing_prefix(path: &Path) -> PathBuf`

# Calls

- [normalize_path](../../../../../functions/src/llm/tools/sandbox/normalize_path.md)

# Called by

- [check](../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/check.md)