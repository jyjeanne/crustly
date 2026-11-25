---
type: Rust Function
title: build_parser_factory
resource: src/llm/provider/llama_cpp_grammar.rs#L118-L122
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/build_grammar_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/test_grammar_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_grammar/schema_compiles_into_a_working_sampler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_grammar/one_factory_builds_multiple_independent_grammars
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_parser_factory( tok_env: &toktrie::TokEnv, ) -> std::result::Result<llguidance::ParserFactory, String>`

# Called by

- [build_grammar_env](../../../../../functions/src/llm/provider/llama_cpp/build_grammar_env.md)
- [test_grammar_env](../../../../../functions/src/llm/provider/llama_cpp/test_grammar_env.md)
- [schema_compiles_into_a_working_sampler](../../../../../functions/src/llm/provider/llama_cpp_grammar/schema_compiles_into_a_working_sampler.md)
- [one_factory_builds_multiple_independent_grammars](../../../../../functions/src/llm/provider/llama_cpp_grammar/one_factory_builds_multiple_independent_grammars.md)