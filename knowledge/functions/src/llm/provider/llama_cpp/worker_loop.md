---
type: Rust Function
title: worker_loop
resource: src/llm/provider/llama_cpp.rs#L266-L437
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp/dispatch_job
  called_by:
  - functions/src/llm/provider/llama_cpp/LlamaCppProvider/new
---

# Signature

`fn worker_loop(init: WorkerInit)`

# Calls

- [dispatch_job](../../../../../functions/src/llm/provider/llama_cpp/dispatch_job.md)

# Called by

- [new](../../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/new.md)