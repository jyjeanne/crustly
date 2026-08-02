---
type: Rust Function
title: build_grammar_env
resource: src/llm/provider/llama_cpp.rs#L1139-L1152
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_grammar/build_parser_factory
---

# Signature

`fn build_grammar_env(model: &LlamaModel) -> Option<ToolCallGrammarEnv>`

# Calls

- [build_parser_factory](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_parser_factory.md)