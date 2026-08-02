---
type: Rust Function
title: list_skills
resource: src/llm/tools/skill.rs#L224-L270
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/skill/skill_lookup_roots
  - functions/src/llm/tools/skill/parse_skill_frontmatter_value
  called_by:
  - functions/src/llm/tools/skill/list_skills_discovers_project_local_skills_with_frontmatter
  - functions/src/llm/tools/skill/list_skills_falls_back_to_directory_name_without_frontmatter_name
  - functions/src/llm/tools/skill/list_skills_discovers_legacy_flat_md_files
  - functions/src/llm/tools/skill/list_skills_is_sorted_alphabetically_case_insensitive
  - functions/src/llm/tools/skill/list_skills_deduplicates_same_name_across_roots
  - functions/src/llm/tools/skill/list_skills_does_not_panic_on_a_directory_with_no_skills_dir
  - functions/src/tui/app/App/open_skills
---

# Signature

`pub(crate) fn list_skills(cwd: &Path) -> Vec<SkillListing>`

# Calls

- [skill_lookup_roots](../../../../../functions/src/llm/tools/skill/skill_lookup_roots.md)
- [parse_skill_frontmatter_value](../../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)

# Called by

- [list_skills_discovers_project_local_skills_with_frontmatter](../../../../../functions/src/llm/tools/skill/list_skills_discovers_project_local_skills_with_frontmatter.md)
- [list_skills_falls_back_to_directory_name_without_frontmatter_name](../../../../../functions/src/llm/tools/skill/list_skills_falls_back_to_directory_name_without_frontmatter_name.md)
- [list_skills_discovers_legacy_flat_md_files](../../../../../functions/src/llm/tools/skill/list_skills_discovers_legacy_flat_md_files.md)
- [list_skills_is_sorted_alphabetically_case_insensitive](../../../../../functions/src/llm/tools/skill/list_skills_is_sorted_alphabetically_case_insensitive.md)
- [list_skills_deduplicates_same_name_across_roots](../../../../../functions/src/llm/tools/skill/list_skills_deduplicates_same_name_across_roots.md)
- [list_skills_does_not_panic_on_a_directory_with_no_skills_dir](../../../../../functions/src/llm/tools/skill/list_skills_does_not_panic_on_a_directory_with_no_skills_dir.md)
- [open_skills](../../../../../functions/src/tui/app/App/open_skills.md)