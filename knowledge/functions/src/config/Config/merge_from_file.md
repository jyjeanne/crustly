---
type: Rust Method
title: merge_from_file
resource: src/config/mod.rs#L861-L869
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/config/secrets/SecretString/from_str
  - functions/src/config/Config/merge
  called_by:
  - functions/src/config/Config/load
  - functions/src/config/Config/load_from_path
---

# Signature

`fn merge_from_file(base: Self, path: &Path) -> Result<Self>`

# Calls

- [with_context](../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [from_str](../../../../functions/src/config/secrets/SecretString/from_str.md)
- [merge](../../../../functions/src/config/Config/merge.md)

# Called by

- [load](../../../../functions/src/config/Config/load.md)
- [load_from_path](../../../../functions/src/config/Config/load_from_path.md)