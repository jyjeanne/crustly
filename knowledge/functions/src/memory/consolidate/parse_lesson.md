---
type: Rust Function
title: parse_lesson
resource: src/memory/consolidate.rs#L260-L266
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/lesson_from_episodes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/a_lesson_is_read_from_the_first_json_object
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_lesson(answer: &str) -> Option<String>`

# Called by

- [lesson_from_episodes](../../../../functions/src/memory/consolidate/MemoryStore/lesson_from_episodes.md)
- [a_lesson_is_read_from_the_first_json_object](../../../../functions/src/memory/consolidate/a_lesson_is_read_from_the_first_json_object.md)