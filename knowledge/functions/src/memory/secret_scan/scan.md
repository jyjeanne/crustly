---
type: Rust Function
title: scan
resource: src/memory/secret_scan.rs#L58-L71
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/secret_scan/shannon_entropy
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/extract/redact_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/only_the_rule_name_is_returned
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/memory/secret_scan/detects_password_assignments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/detects_authorization_and_cookie_headers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/detects_a_high_entropy_token
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/ordinary_text_passes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/commit_hashes_and_uuids_pass
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn scan(text: &str) -> Option<&'static str>`

# Calls

- [shannon_entropy](../../../../functions/src/memory/secret_scan/shannon_entropy.md)

# Called by

- [redact_line](../../../../functions/src/memory/extract/redact_line.md)
- [save_with](../../../../functions/src/memory/MemoryStore/save_with.md)
- [edit](../../../../functions/src/memory/MemoryStore/edit.md)
- [only_the_rule_name_is_returned](../../../../functions/src/memory/secret_scan/only_the_rule_name_is_returned.md)
- [detects_a_private_key_header](../../../../functions/src/memory/secret_scan/detects_a_private_key_header.md)
- [detects_an_aws_access_key](../../../../functions/src/memory/secret_scan/detects_an_aws_access_key.md)
- [detects_github_tokens](../../../../functions/src/memory/secret_scan/detects_github_tokens.md)
- [detects_provider_api_keys](../../../../functions/src/memory/secret_scan/detects_provider_api_keys.md)
- [detects_slack_and_google_keys](../../../../functions/src/memory/secret_scan/detects_slack_and_google_keys.md)
- [detects_a_jwt](../../../../functions/src/memory/secret_scan/detects_a_jwt.md)
- [detects_credentials_in_a_url](../../../../functions/src/memory/secret_scan/detects_credentials_in_a_url.md)
- [detects_password_assignments](../../../../functions/src/memory/secret_scan/detects_password_assignments.md)
- [detects_authorization_and_cookie_headers](../../../../functions/src/memory/secret_scan/detects_authorization_and_cookie_headers.md)
- [detects_a_high_entropy_token](../../../../functions/src/memory/secret_scan/detects_a_high_entropy_token.md)
- [ordinary_text_passes](../../../../functions/src/memory/secret_scan/ordinary_text_passes.md)
- [commit_hashes_and_uuids_pass](../../../../functions/src/memory/secret_scan/commit_hashes_and_uuids_pass.md)