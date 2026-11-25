---
type: Rust Function
title: parse_skill_frontmatter_value
resource: src/llm/tools/skill.rs#L367-L390
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/skill/SkillTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/frontmatter_name_matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/test_parse_frontmatter_description
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/test_parse_frontmatter_no_frontmatter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/test_parse_frontmatter_missing_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_skill_frontmatter_value(contents: &str, key: &str) -> Option<String>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/skill/SkillTool/tool/execute.md)
- [resolve_skill_content](../../../../../functions/src/llm/tools/skill/resolve_skill_content.md)
- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)
- [frontmatter_name_matches](../../../../../functions/src/llm/tools/skill/frontmatter_name_matches.md)
- [test_parse_frontmatter_description](../../../../../functions/src/llm/tools/skill/test_parse_frontmatter_description.md)
- [test_parse_frontmatter_no_frontmatter](../../../../../functions/src/llm/tools/skill/test_parse_frontmatter_no_frontmatter.md)
- [test_parse_frontmatter_missing_key](../../../../../functions/src/llm/tools/skill/test_parse_frontmatter_missing_key.md)