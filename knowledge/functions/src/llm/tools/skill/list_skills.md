---
type: Rust Function
title: list_skills
resource: src/llm/tools/skill.rs#L299-L358
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/skill/skill_lookup_roots
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/parse_skill_frontmatter_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/lookup
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/skill/list_skills_discovers_project_local_skills_with_frontmatter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_falls_back_to_directory_name_without_frontmatter_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_discovers_legacy_flat_md_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_is_sorted_alphabetically_case_insensitive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_deduplicates_same_name_across_roots
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_does_not_panic_on_a_directory_with_no_skills_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_includes_builtin_review_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_includes_builtin_spec_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_lets_project_skill_shadow_builtin_of_same_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/open_skills
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn list_skills(cwd: &Path) -> Vec<SkillListing>`

# Calls

- [skill_lookup_roots](../../../../../functions/src/llm/tools/skill/skill_lookup_roots.md)
- [parse_skill_frontmatter_value](../../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)
- [lookup](../../../../../functions/src/llm/tools/skill/lookup.md)

# Called by

- [list_skills_discovers_project_local_skills_with_frontmatter](../../../../../functions/src/llm/tools/skill/list_skills_discovers_project_local_skills_with_frontmatter.md)
- [list_skills_falls_back_to_directory_name_without_frontmatter_name](../../../../../functions/src/llm/tools/skill/list_skills_falls_back_to_directory_name_without_frontmatter_name.md)
- [list_skills_discovers_legacy_flat_md_files](../../../../../functions/src/llm/tools/skill/list_skills_discovers_legacy_flat_md_files.md)
- [list_skills_is_sorted_alphabetically_case_insensitive](../../../../../functions/src/llm/tools/skill/list_skills_is_sorted_alphabetically_case_insensitive.md)
- [list_skills_deduplicates_same_name_across_roots](../../../../../functions/src/llm/tools/skill/list_skills_deduplicates_same_name_across_roots.md)
- [list_skills_does_not_panic_on_a_directory_with_no_skills_dir](../../../../../functions/src/llm/tools/skill/list_skills_does_not_panic_on_a_directory_with_no_skills_dir.md)
- [list_skills_includes_builtin_review_skill](../../../../../functions/src/llm/tools/skill/list_skills_includes_builtin_review_skill.md)
- [list_skills_includes_builtin_spec_skill](../../../../../functions/src/llm/tools/skill/list_skills_includes_builtin_spec_skill.md)
- [list_skills_lets_project_skill_shadow_builtin_of_same_name](../../../../../functions/src/llm/tools/skill/list_skills_lets_project_skill_shadow_builtin_of_same_name.md)
- [open_skills](../../../../../functions/src/tui/app/App/open_skills.md)