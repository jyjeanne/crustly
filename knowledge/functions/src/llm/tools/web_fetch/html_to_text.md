---
type: Rust Function
title: html_to_text
resource: src/llm/tools/web_fetch.rs#L56-L78
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
  - functions/src/llm/tools/web_fetch/test_html_to_text_strips_tags
  - functions/src/llm/tools/web_fetch/test_html_to_text_strips_script
  - functions/src/llm/tools/web_fetch/test_html_to_text_decodes_entities
---

# Signature

`fn html_to_text(html: &str) -> String`

# Called by

- [execute](../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)
- [test_html_to_text_strips_tags](../../../../../functions/src/llm/tools/web_fetch/test_html_to_text_strips_tags.md)
- [test_html_to_text_strips_script](../../../../../functions/src/llm/tools/web_fetch/test_html_to_text_strips_script.md)
- [test_html_to_text_decodes_entities](../../../../../functions/src/llm/tools/web_fetch/test_html_to_text_decodes_entities.md)