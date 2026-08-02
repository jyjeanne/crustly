---
type: Rust Method
title: resolve
resource: src/llm/tools/ssrf_guard.rs#L87-L126
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/ssrf_guard/is_blocked_ip
  - functions/src/config/secrets/SecretString/is_empty
---

# Signature

`fn resolve(&self, name: Name) -> Resolving`

# Calls

- [is_blocked_ip](../../../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)