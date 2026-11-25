---
type: Rust Module
title: profiles
resource: src/llm/agent/profiles.rs#L1-L182
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crate-config-agentprofileconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-btreemap-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [AgentProfile](../../../../classes/src/llm/agent/profiles/AgentProfile.md)
- [names](../../../../functions/src/llm/agent/profiles/names.md)
- [builtin_profiles](../../../../functions/src/llm/agent/profiles/builtin_profiles.md)
- [resolve_profiles](../../../../functions/src/llm/agent/profiles/resolve_profiles.md)
- [the_built_in_profiles_exist_and_explorers_cannot_write](../../../../functions/src/llm/agent/profiles/the_built_in_profiles_exist_and_explorers_cannot_write.md)
- [configuration_overrides_fields_and_adds_profiles](../../../../functions/src/llm/agent/profiles/configuration_overrides_fields_and_adds_profiles.md)

# Imports

- `crate::config::AgentProfileConfig`
- `std::collections::{BTreeMap, HashMap}`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)