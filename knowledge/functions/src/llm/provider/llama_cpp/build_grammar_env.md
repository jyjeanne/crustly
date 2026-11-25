---
type: Rust Function
title: build_grammar_env
resource: src/llm/provider/llama_cpp.rs#L1233-L1246
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_grammar/build_parser_factory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_grammar_env(model: &LlamaModel) -> Option<ToolCallGrammarEnv>`

# Calls

- [build_parser_factory](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_parser_factory.md)