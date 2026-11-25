---
type: Rust Function
title: loser
resource: src/memory/semantic.rs#L381-L383
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/semantic/near_duplicate_to_archive
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/semantic/the_newer_near_duplicate_stays
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/a_user_memory_beats_a_newer_one_the_user_did_not_save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/episodes_and_different_categories_are_both_kept
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn loser(a: &Memory, b: &Memory) -> Option<Uuid>`

# Calls

- [near_duplicate_to_archive](../../../../functions/src/memory/semantic/near_duplicate_to_archive.md)

# Called by

- [the_newer_near_duplicate_stays](../../../../functions/src/memory/semantic/the_newer_near_duplicate_stays.md)
- [a_user_memory_beats_a_newer_one_the_user_did_not_save](../../../../functions/src/memory/semantic/a_user_memory_beats_a_newer_one_the_user_did_not_save.md)
- [episodes_and_different_categories_are_both_kept](../../../../functions/src/memory/semantic/episodes_and_different_categories_are_both_kept.md)