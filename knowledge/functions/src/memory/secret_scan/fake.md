---
type: Rust Function
title: fake
resource: src/memory/secret_scan.rs#L100-L102
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/secret_scan/detects_a_private_key_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/detects_an_aws_access_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/detects_github_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/detects_provider_api_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/detects_slack_and_google_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/detects_a_jwt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/detects_credentials_in_a_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/detects_authorization_and_cookie_headers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/only_the_rule_name_is_returned
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fake(parts: &[&str]) -> String`

# Called by

- [detects_a_private_key_header](../../../../functions/src/memory/secret_scan/detects_a_private_key_header.md)
- [detects_an_aws_access_key](../../../../functions/src/memory/secret_scan/detects_an_aws_access_key.md)
- [detects_github_tokens](../../../../functions/src/memory/secret_scan/detects_github_tokens.md)
- [detects_provider_api_keys](../../../../functions/src/memory/secret_scan/detects_provider_api_keys.md)
- [detects_slack_and_google_keys](../../../../functions/src/memory/secret_scan/detects_slack_and_google_keys.md)
- [detects_a_jwt](../../../../functions/src/memory/secret_scan/detects_a_jwt.md)
- [detects_credentials_in_a_url](../../../../functions/src/memory/secret_scan/detects_credentials_in_a_url.md)
- [detects_authorization_and_cookie_headers](../../../../functions/src/memory/secret_scan/detects_authorization_and_cookie_headers.md)
- [only_the_rule_name_is_returned](../../../../functions/src/memory/secret_scan/only_the_rule_name_is_returned.md)