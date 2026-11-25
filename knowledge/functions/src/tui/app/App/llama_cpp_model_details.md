---
type: Rust Method
title: llama_cpp_model_details
resource: src/tui/app.rs#L931-L950
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/render/render_model_info
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn llama_cpp_model_details( &self, ) -> Option<super::llama_cpp_download::LlamaCppModelDetails>`

# Calls

- [provider_name](../../../../../functions/src/tui/app/App/provider_name.md)

# Called by

- [render_model_info](../../../../../functions/src/tui/render/render_model_info.md)