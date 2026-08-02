---
type: Rust Module
title: skill
resource: src/llm/tools/skill.rs#L1-L459
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-path-path-pathbuf
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [SkillTool](../../../../classes/src/llm/tools/skill/SkillTool.md)
- [SkillInput](../../../../classes/src/llm/tools/skill/SkillInput.md)
- [SkillOutput](../../../../classes/src/llm/tools/skill/SkillOutput.md)
- [name](../../../../functions/src/llm/tools/skill/SkillTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/skill/SkillTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/skill/SkillTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/skill/SkillTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/skill/SkillTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/skill/SkillTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/skill/SkillTool/tool/execute.md)
- [resolve_skill_path](../../../../functions/src/llm/tools/skill/resolve_skill_path.md)
- [skill_lookup_roots](../../../../functions/src/llm/tools/skill/skill_lookup_roots.md)
- [push_if_dir](../../../../functions/src/llm/tools/skill/push_if_dir.md)
- [SkillListing](../../../../classes/src/llm/tools/skill/SkillListing.md)
- [list_skills](../../../../functions/src/llm/tools/skill/list_skills.md)
- [frontmatter_name_matches](../../../../functions/src/llm/tools/skill/frontmatter_name_matches.md)
- [parse_skill_frontmatter_value](../../../../functions/src/llm/tools/skill/parse_skill_frontmatter_value.md)
- [test_parse_frontmatter_description](../../../../functions/src/llm/tools/skill/test_parse_frontmatter_description.md)
- [test_parse_frontmatter_no_frontmatter](../../../../functions/src/llm/tools/skill/test_parse_frontmatter_no_frontmatter.md)
- [test_parse_frontmatter_missing_key](../../../../functions/src/llm/tools/skill/test_parse_frontmatter_missing_key.md)
- [test_validate_empty_skill_name](../../../../functions/src/llm/tools/skill/test_validate_empty_skill_name.md)
- [test_validate_valid_skill_name](../../../../functions/src/llm/tools/skill/test_validate_valid_skill_name.md)
- [test_validate_rejects_dotdot_traversal](../../../../functions/src/llm/tools/skill/test_validate_rejects_dotdot_traversal.md)
- [test_validate_allows_namespaced_skill](../../../../functions/src/llm/tools/skill/test_validate_allows_namespaced_skill.md)
- [test_validate_rejects_null_byte](../../../../functions/src/llm/tools/skill/test_validate_rejects_null_byte.md)
- [list_skills_discovers_project_local_skills_with_frontmatter](../../../../functions/src/llm/tools/skill/list_skills_discovers_project_local_skills_with_frontmatter.md)
- [list_skills_falls_back_to_directory_name_without_frontmatter_name](../../../../functions/src/llm/tools/skill/list_skills_falls_back_to_directory_name_without_frontmatter_name.md)
- [list_skills_discovers_legacy_flat_md_files](../../../../functions/src/llm/tools/skill/list_skills_discovers_legacy_flat_md_files.md)
- [list_skills_is_sorted_alphabetically_case_insensitive](../../../../functions/src/llm/tools/skill/list_skills_is_sorted_alphabetically_case_insensitive.md)
- [list_skills_deduplicates_same_name_across_roots](../../../../functions/src/llm/tools/skill/list_skills_deduplicates_same_name_across_roots.md)
- [list_skills_does_not_panic_on_a_directory_with_no_skills_dir](../../../../functions/src/llm/tools/skill/list_skills_does_not_panic_on_a_directory_with_no_skills_dir.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::path::{Path, PathBuf}`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)