---
type: Rust Method
title: check
resource: src/llm/tools/sandbox.rs#L129-L186
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/normalize_path
  - functions/src/llm/tools/sandbox/strip_verbatim_prefix
  - functions/src/llm/tools/sandbox/resolve_existing_prefix
---

# Signature

`fn check(&self, raw: &str) -> PolicyDecision`

# Calls

- [normalize_path](../../../../../../functions/src/llm/tools/sandbox/normalize_path.md)
- [strip_verbatim_prefix](../../../../../../functions/src/llm/tools/sandbox/strip_verbatim_prefix.md)
- [resolve_existing_prefix](../../../../../../functions/src/llm/tools/sandbox/resolve_existing_prefix.md)