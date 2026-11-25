---
type: Rust Method
title: local
resource: src/llm/provider/openai.rs#L89-L105
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/factory/try_create_openai
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_local_provider_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_supports_vision_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn local(base_url: String) -> Self`

# Calls

- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)

# Called by

- [try_create_openai](../../../../../../functions/src/llm/provider/factory/try_create_openai.md)
- [test_local_provider_creation](../../../../../../functions/src/llm/provider/openai/test_local_provider_creation.md)
- [test_supports_vision_detection](../../../../../../functions/src/llm/provider/openai/test_supports_vision_detection.md)
- [text_arrives_before_the_response_is_complete](../../../../../../functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete.md)