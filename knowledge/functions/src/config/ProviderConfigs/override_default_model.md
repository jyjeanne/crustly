---
type: Rust Method
title: override_default_model
resource: src/config/mod.rs#L797-L829
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/model_override_targets_the_selected_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/model_override_respects_provider_precedence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/model_override_reports_when_no_provider_can_take_it
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/model_override_skips_disabled_providers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/model_override_targets_gemini_when_it_is_the_selected_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/model_override_skips_gemini_without_api_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn override_default_model(&mut self, model: &str) -> Option<&'static str>`

# Called by

- [run](../../../../functions/src/cli/run.md)
- [model_override_targets_the_selected_provider](../../../../functions/src/config/model_override_targets_the_selected_provider.md)
- [model_override_respects_provider_precedence](../../../../functions/src/config/model_override_respects_provider_precedence.md)
- [model_override_reports_when_no_provider_can_take_it](../../../../functions/src/config/model_override_reports_when_no_provider_can_take_it.md)
- [model_override_skips_disabled_providers](../../../../functions/src/config/model_override_skips_disabled_providers.md)
- [model_override_targets_gemini_when_it_is_the_selected_provider](../../../../functions/src/config/model_override_targets_gemini_when_it_is_the_selected_provider.md)
- [model_override_skips_gemini_without_api_key](../../../../functions/src/config/model_override_skips_gemini_without_api_key.md)