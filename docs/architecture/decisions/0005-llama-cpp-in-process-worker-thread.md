# 0005. In-process llama.cpp provider: one dedicated OS worker thread per model

Status: Accepted

## Context

Every existing `Provider` implementation (Anthropic, OpenAI, Gemini,
Qwen, Azure, native Ollama) is an HTTP client talking to a process that is
already running — the LLM inference itself happens somewhere else, and
`crustly` just sends and receives bytes over the network. Adding
`providers.llama_cpp` (via the [`llama-cpp-2`](https://github.com/utilityai/llama-cpp-rs)
crate, FFI bindings to `llama.cpp`) changes that: model weights are loaded
and inference runs **inside the `crustly` process itself**. That raises a
question none of the other providers had to answer: what owns the loaded
model and the mutable inference state, and on what thread does it run?

Two structural facts about `llama-cpp-2` (confirmed by reading the crate's
source directly, not assumed) rule out the obvious answer of "just call it
from wherever, like any other async function":

1. **`LlamaContext` and `LlamaBackend` are not `Send`/`Sync`.** Unlike
   `LlamaModel`, which the crate explicitly marks `unsafe impl Send` /
   `unsafe impl Sync` (read-only weights, safe to share), neither
   `LlamaContext` (the mutable KV-cache/decode state) nor `LlamaBackend`
   carries such an impl. They cannot be moved across threads or shared
   across `.await` points the way a `tokio::task::spawn_blocking` call
   per-request would require.
2. **Reusing the KV cache across turns of the same conversation matters.**
   Tearing down and rebuilding a `LlamaContext` per request (the natural
   consequence of a `spawn_blocking`-per-call design) would mean
   re-processing the entire prompt from scratch on every turn, discarding
   the whole benefit of an already-warm context.

Also relevant: this codebase already has a working precedent for exactly
this shape of problem. `crate::app::start_file_watcher`
(`src/app/mod.rs`) needed to run a synchronous, non-`Send`-friendly
library (`notify::Watcher`) without blocking the async runtime, and solved
it by spawning a dedicated `std::thread::spawn` OS thread and bridging its
output into async code via a `tokio::sync::mpsc` channel — "Spawn the
synchronous watcher on a dedicated OS thread (not tokio)," per that
module's own doc comment.

Alternatives considered:

- **`tokio::task::spawn_blocking` per request.** Rejected: doesn't satisfy
  `LlamaContext`'s `!Send` constraint without also re-creating the context
  every call (see point 2 above), and would let multiple concurrent
  requests race to construct competing contexts against the same
  model/GPU memory, which `llama.cpp` is not designed for.
- **A process-per-model sidecar (reintroducing an HTTP boundary).** Rejected
  as contrary to the entire point of this feature — the reason to build
  this at all instead of just using the native Ollama provider is to
  *not* need a separate process.
- **A shared `Mutex<LlamaContext>` accessed from arbitrary async tasks.**
  Rejected: still requires `Send`, which `LlamaContext` doesn't have, and
  would serialize requests behind a lock without the clean lifecycle
  control (load once, idle-unload, panic isolation) a dedicated thread
  gives for free.

## Decision

`LlamaCppProvider` owns a single dedicated OS thread (spawned via
`std::thread::spawn`, not a tokio task) that is the *only* thing that ever
calls into `llama-cpp-2`. `LlamaBackend`, `LlamaModel`, and `LlamaContext`
are created, live, and are dropped exclusively on that thread, as sibling
local variables in one stack frame — never stored as fields shared across
an `Option`/struct boundary, which `LlamaContext`'s lifetime (it borrows
from `LlamaModel`) would make self-referential and require `unsafe`
(`self_cell`/`ouroboros`-style) to express safely.

Requests cross the thread boundary via a `tokio::sync::mpsc::UnboundedSender<InferenceJob>`
from the async side; the worker thread drains it with `blocking_recv()` —
the same bridge shape `start_file_watcher` already established for another
non-async library, just with the data direction reversed (there, a sync
thread pushes events out to an async consumer; here, async callers push
jobs in to a sync thread). Non-streaming responses return through a
`oneshot` channel per request; streaming responses push `StreamEvent`s
incrementally through a second `mpsc` channel, so `Provider::stream()`
returns before generation starts rather than buffering the whole response.

Because only one worker thread exists per provider instance, and it
processes jobs strictly one at a time, two concurrent chat sessions
pointed at the same `LlamaCppProvider` serialize rather than run in
parallel. This is treated as an accepted limitation, not a bug to
engineer around: it matches the real hardware constraint (one GPU/CPU
worth of compute) rather than fighting it, and multiple concurrent
generations against one model on one device wouldn't usefully parallelize
even with a different design.

Model loading itself happens **on the worker thread**, not the caller's —
`LlamaCppProvider::new()` blocks (via a `oneshot`) until the worker
confirms the model loaded (or reports why it didn't) before returning, so
callers get a meaningful `Result` without any FFI call ever running off
the worker thread, including the very first one.

Idle-unload (`providers.llama_cpp.idle_unload_secs`) reuses this same
scoping trick: the worker's main loop is structured as an outer "load,
then serve" loop rather than a single load followed by one serve pass. An
idle timeout simply lets one iteration's `backend`/`model`/`context` go
out of scope; the next iteration reloads them fresh. No persistent
"unloaded" state needs representing across an `Option` boundary — the
absence of a loaded model between iterations *is* the unloaded state.

## Consequences

**Easier:**
- KV-cache reuse across a conversation's turns without re-processing the
  whole prompt each time.
- A single, auditable point (the worker thread body) where every FFI call
  into `llama.cpp` happens, making the `catch_unwind` panic-isolation
  boundary (a native-code panic must not take the whole process down, per
  `llama-cpp-2-integration-plan.md` §4.11) trivial to apply completely,
  rather than needing to wrap every call site individually.
- Idle-unload and reload require no unsafe code and no extra dependency
  (`self_cell` et al.), because the self-referential
  `LlamaContext<'model>` problem never has to be solved — it's sidestepped
  by scope, not worked around.

**Harder:**
- Two chat sessions sharing one `LlamaCppProvider` instance cannot
  generate concurrently — requests queue behind the worker thread's single
  serving loop. Documented as accepted, not silently absorbed.
- Model swap (switching `model_path`) cannot be a cheap in-place update
  the way it is for `OllamaProvider` (which just changes which model name
  a request sends to an already-running server). It requires tearing down
  and reconstructing the whole `LlamaCppProvider` — a multi-second-to-
  tens-of-seconds operation depending on file size and disk speed. Any
  future TUI model-switch UX for this provider (`llama-cpp-2-integration-plan.md`
  §7 point 4) must show a blocking "Loading model…" state, not the
  near-instant swap Ollama's dialog gives today.
- Debugging FFI-boundary issues means reasoning about a plain OS thread
  with its own panic/unwind semantics, distinct from the async task
  cancellation and `.await`-point semantics the rest of the codebase's
  concurrency relies on — a different mental model to hold when working on
  this provider specifically.

See `llama-cpp-2-integration-plan.md` (repo root) for the full technical
design this ADR summarizes the load-bearing decision from, and
`llm-file-gguf-support.md` for the original benefit/cost feasibility study
and Go/No-Go framework this feature was built under.
