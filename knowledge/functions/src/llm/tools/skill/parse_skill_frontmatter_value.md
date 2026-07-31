---
type: Rust Function
title: parse_skill_frontmatter_value
resource: src/llm/tools/skill.rs#L279-L302
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/skill/SkillTool/tool/execute
  - functions/src/llm/tools/skill/list_skills
  - functions/src/llm/tools/skill/frontmatter_name_matches
---

# Signature

`fn parse_skill_frontmatter_value(contents: &str, key: &str) -> Option<String>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/skill/SkillTool/tool/execute.md)
- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)
- [frontmatter_name_matches](../../../../../functions/src/llm/tools/skill/frontmatter_name_matches.md)