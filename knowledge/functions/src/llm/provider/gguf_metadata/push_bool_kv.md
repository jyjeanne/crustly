---
type: Rust Function
title: push_bool_kv
resource: src/llm/provider/gguf_metadata.rs#L559-L563
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_true_sets_is_vision_projector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_false_does_not_set_is_vision_projector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/clip_has_audio_encoder_true_sets_is_audio_projector
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn push_bool_kv(buf: &mut Vec<u8>, key: &str, value: bool)`

# Called by

- [clip_has_vision_encoder_true_sets_is_vision_projector](../../../../../functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_true_sets_is_vision_projector.md)
- [clip_has_vision_encoder_false_does_not_set_is_vision_projector](../../../../../functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_false_does_not_set_is_vision_projector.md)
- [clip_has_audio_encoder_true_sets_is_audio_projector](../../../../../functions/src/llm/provider/gguf_metadata/clip_has_audio_encoder_true_sets_is_audio_projector.md)