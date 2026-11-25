---
type: Rust Module
title: secret_scan
resource: src/memory/secret_scan.rs#L1-L206
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [scan](../../../functions/src/memory/secret_scan/scan.md)
- [shannon_entropy](../../../functions/src/memory/secret_scan/shannon_entropy.md)
- [fake](../../../functions/src/memory/secret_scan/fake.md)
- [detects_a_private_key_header](../../../functions/src/memory/secret_scan/detects_a_private_key_header.md)
- [detects_an_aws_access_key](../../../functions/src/memory/secret_scan/detects_an_aws_access_key.md)
- [detects_github_tokens](../../../functions/src/memory/secret_scan/detects_github_tokens.md)
- [detects_provider_api_keys](../../../functions/src/memory/secret_scan/detects_provider_api_keys.md)
- [detects_slack_and_google_keys](../../../functions/src/memory/secret_scan/detects_slack_and_google_keys.md)
- [detects_a_jwt](../../../functions/src/memory/secret_scan/detects_a_jwt.md)
- [detects_credentials_in_a_url](../../../functions/src/memory/secret_scan/detects_credentials_in_a_url.md)
- [detects_password_assignments](../../../functions/src/memory/secret_scan/detects_password_assignments.md)
- [detects_authorization_and_cookie_headers](../../../functions/src/memory/secret_scan/detects_authorization_and_cookie_headers.md)
- [detects_a_high_entropy_token](../../../functions/src/memory/secret_scan/detects_a_high_entropy_token.md)
- [ordinary_text_passes](../../../functions/src/memory/secret_scan/ordinary_text_passes.md)
- [commit_hashes_and_uuids_pass](../../../functions/src/memory/secret_scan/commit_hashes_and_uuids_pass.md)
- [only_the_rule_name_is_returned](../../../functions/src/memory/secret_scan/only_the_rule_name_is_returned.md)

# Imports

- `regex::Regex`
- `std::sync::LazyLock`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)