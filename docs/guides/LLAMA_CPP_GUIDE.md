# ⚙️ Using Crustly with llama.cpp (in-process, no server)

## What is this?

Every other local-LLM path in Crustly — native Ollama (`providers.ollama`),
LM Studio, or Ollama's OpenAI-compatible shim (`providers.openai.base_url`)
— talks to a **separate process** that's already running. This one doesn't:
`providers.llama_cpp` loads a `.gguf` model file directly into the
`crustly` process itself, via the [`llama-cpp-2`](https://github.com/utilityai/llama-cpp-rs)
crate (Rust bindings to [`llama.cpp`](https://github.com/ggerganov/llama.cpp)).
There is no daemon to install, no port to start, no second thing to keep
running — just Crustly and a model file.

**Use this if:**
- You already have a `.gguf` file and don't want a second copy duplicated
  into Ollama's own model store.
- You want zero background processes — no `ollama serve`, no LM Studio
  window, nothing listening on a local port.
- You want direct control over GPU offload (`n_gpu_layers`) and threading
  without going through Ollama's own defaults.

**Use Ollama or LM Studio instead if:**
- You want the easiest path (`ollama pull <name>` and go) — this guide's
  path requires building Crustly from source with an extra Cargo feature.
- You want to run several Crustly sessions (or other tools) against the
  *same* loaded model at once — Ollama's server shares one loaded model
  across clients; each `providers.llama_cpp` instance loads its own copy.
- You want to switch models frequently mid-session — Ollama's server swap
  is near-instant; here, switching models means unloading and reloading a
  multi-GB file (seconds to tens of seconds).

See [`llama-cpp-2-integration-plan.md`](../../llama-cpp-2-integration-plan.md)
(repo root) for the full design and [`llm-file-gguf-support.md`](../../llm-file-gguf-support.md)
for the original feasibility study, including a documented, currently-unmet
Go/No-Go gate — this feature exists and works, but the study's own
recommendation was conditional. Read §0.1 of the integration plan before
recommending this path to end users at scale.

## Build requirements (read this before enabling)

Unlike every other provider, `providers.llama_cpp` needs a build feature
that compiles native C++:

```bash
cargo build --release --features llama-cpp
```

This requires, on the machine doing the build:

- A C and C++ toolchain (`cc`, `cmake` ≥ 3.14). On Linux this is usually
  already present or a single package install (`build-essential` +
  `cmake` on Debian/Ubuntu, or your distro's equivalent). On **Windows**
  you need Visual Studio Build Tools (C++ workload) or MinGW — there's no
  toolchain present by default the way there effectively is on
  Linux/macOS. On **macOS**, Xcode command-line tools (`xcode-select
  --install`).
- Nothing else for CPU-only inference (the default). GPU backends need
  their own SDK — see [GPU acceleration](#gpu-acceleration) below.

The first build with this feature compiles `llama.cpp` from source, which
takes noticeably longer than a normal Crustly build (typically under a
minute on modern hardware with a few cores, CPU-only — GPU backends add
more). Subsequent builds are incremental as usual.

This feature is **not** part of `--features all-llm` and never will be by
default — it's the only provider feature with a native-toolchain
requirement, and keeping it opt-in keeps the default/`all-llm` build fast
and hermetic for everyone who doesn't need it.

## Getting a model

You need a `.gguf` file. Two ways to get one:

### Option A — already have one

If you've downloaded a `.gguf` from Hugging Face, converted one yourself,
or have one from another tool (LM Studio's model folder, for instance),
just point `model_path` at it — see [Configuration](#configuration) below.

### Option B — download via Crustly

```bash
# Direct URL
crustly llama-cpp pull https://example.com/path/to/model.gguf

# Hugging Face shorthand - resolves to
# https://huggingface.co/<org>/<repo>/resolve/main/<file>
crustly llama-cpp pull hf:Qwen/Qwen2.5-Coder-7B-Instruct-GGUF/qwen2.5-coder-7b-instruct-q4_k_m.gguf
```

The `hf:` shorthand also looks up the file's published SHA-256 (Hugging
Face publishes this for LFS-tracked files, which `.gguf` files always are
given their size) and verifies the download against it — a mismatch
deletes the partial file and reports both hashes rather than leaving a
corrupted file behind. A direct URL has no metadata endpoint to check
against, so it downloads with a warning that no integrity hash is
available. **Gated/private Hugging Face repos aren't supported** — there's
no token support in this version.

Downloaded models land in `providers.llama_cpp.models_dir` (default: a
platform cache directory, `~/.cache/crustly/models` on Linux).

```bash
# List what's downloaded
crustly llama-cpp list

# Remove one (asks for confirmation - a self-downloaded multi-GB file is
# harder to reacquire than an `ollama pull` re-fetch from a registry)
crustly llama-cpp rm qwen2.5-coder-7b-instruct-q4_k_m.gguf
```

There is no `crustly llama-cpp show` (unlike Ollama) — `llama.cpp` has no
equivalent of Ollama's `/api/show` metadata endpoint. `crustly llama-cpp
list` shows a best-effort quantization guess parsed from the filename
convention (e.g. `Q4_K_M`, `Q8_0`) instead.

## Configuration

Minimal `config.toml`:

```toml
[providers.llama_cpp]
enabled = true
model_path = "/home/you/.cache/crustly/models/qwen2.5-coder-7b-instruct-q4_k_m.gguf"
```

Everything else has a documented default — see the commented
`[providers.llama_cpp]` block in `config.toml.example` for the full field
list (context size, sampling, GPU layers, idle-unload, models directory).
A few worth calling out:

- **`n_ctx`** is fixed for the life of the loaded context, unlike Ollama's
  per-request `num_ctx` — changing it requires a model reload.
- **`idle_unload_secs`**: if set, the model is unloaded from memory after
  this many idle seconds and reloaded lazily on the next request (paying
  the load cost again). Unset means it stays loaded until Crustly exits.
- **`chat_template`**: normally unnecessary — the model's own embedded GGUF
  chat template is used automatically. Set this only if a model ships
  without a usable one, or the embedded one doesn't match how it was
  actually fine-tuned.

## GPU acceleration

CPU-only is the default and requires nothing extra. To offload layers to a
GPU, build with the matching feature *and* its SDK installed:

| Feature | Hardware | Requires |
|---|---|---|
| `llama-cpp-cuda` | NVIDIA | CUDA toolkit matching your driver |
| `llama-cpp-metal` | Apple Silicon / Apple GPUs | Xcode command-line tools (macOS only) |
| `llama-cpp-vulkan` | Most GPUs (cross-vendor) | Vulkan SDK (`VULKAN_SDK` discoverable) |
| `llama-cpp-rocm` | AMD (Linux) | ROCm toolkit |
| `llama-cpp-opencl` | Broad/older hardware | An OpenCL SDK/runtime |
| `llama-cpp-mkl` | Intel CPUs (not a GPU backend) | Intel oneMKL |

```bash
cargo build --release --features llama-cpp-cuda
```

Then set `n_gpu_layers` above `0` in `[providers.llama_cpp]` — how many
layers to offload depends on your model size and available VRAM; start
low and increase until you hit a memory error, or set it high (e.g. 999)
to offload everything that fits. **If you set `n_gpu_layers > 0` without
building with a matching feature, Crustly logs a warning at startup and
silently runs CPU-only** — the setting isn't an error, but it's a no-op.

Approximate VRAM needed scales with model size and quantization — as a
rough guide, a Q4_K_M-quantized model needs roughly half its parameter
count in GB of VRAM to fully offload (a 7B Q4_K_M model needs roughly
4-5 GB), with less-aggressive quantizations (Q8_0, F16) needing
correspondingly more. Partial offload (some layers on GPU, the rest on
CPU) is also viable via `n_gpu_layers` set below the model's full layer
count.

## What's different from Ollama, concretely

| | Native Ollama | `providers.llama_cpp` |
|---|---|---|
| Background process | Yes (`ollama serve`) | No |
| Model swap cost | Cheap, near-instant | Multi-GB file reload (seconds+) |
| Multiple clients sharing one loaded model | Yes | No — each instance loads its own copy |
| Idle memory when not in use | ~1GB+ (daemon), more with a model resident | Zero — nothing running outside an active Crustly session |
| Tool calling | Native for supporting models, text-recovery fallback otherwise | Text-recovery only (same mechanism, shared code) |
| Model catalog | Named tags (`ollama pull qwen2.5-coder:7b`) | Manual file management |

Both can be configured at once — `providers.ollama` and
`providers.llama_cpp` don't conflict. `create_provider()`'s resolution
order tries Qwen, then native Ollama, then llama.cpp, then OpenAI-compatible,
then Gemini, then Azure, then Anthropic — the first one with a valid,
enabled config wins.

## Troubleshooting

#### "llama.cpp model file not found"

`model_path` doesn't point to an existing file. Check the path is correct
and absolute (relative paths are resolved against Crustly's working
directory, which may not be what you expect).

#### Build fails with a cmake/compiler error

Missing the C/C++ toolchain — see [Build requirements](#build-requirements-read-this-before-enabling)
above. The exact error usually names what's missing (no `cmake` found, no
C++ compiler found, etc.).

#### "This build of crustly was compiled without the 'llama-cpp' feature"

You're running a binary built without `--features llama-cpp`. Rebuild with
it, or use `providers.ollama`/`providers.openai.base_url` instead if you
don't need this specific path.

#### Out of memory / crash on load with `n_gpu_layers` set high

Your GPU doesn't have enough VRAM for that many offloaded layers. Lower
`n_gpu_layers` (partial offload is fine) or switch to a smaller/more
aggressively quantized model.

#### Responses are slow

Confirm you're actually using a GPU build if you expect GPU speed (check
the [GPU acceleration](#gpu-acceleration) warning isn't showing at
startup), and that `n_threads` is set sensibly for CPU-only inference
(defaults to the number of logical cores if unset).

#### A crash takes down the whole Crustly process

Unlike Ollama (a separate process — a bad response just drops the HTTP
connection), `llama.cpp` runs natively inside the Crustly process. Rust
panics inside the FFI boundary are caught and reported as an error without
crashing the process, but genuine memory-unsafety in the native library
(triggered by a corrupt or malicious `.gguf` file, for instance) is not
something Rust-level panic handling can catch. Only load `.gguf` files you
trust, the same posture as running any other native binary/library against
untrusted input — see `llama-cpp-2-integration-plan.md` §4.11 for the full
threat-model discussion.

## See also

- [`OLLAMA_GUIDE.md`](./OLLAMA_GUIDE.md) — the server-based alternative,
  recommended for most users.
- [`llama-cpp-2-integration-plan.md`](../../llama-cpp-2-integration-plan.md)
  — full technical design, phasing, and known gaps.
- [`llm-file-gguf-support.md`](../../llm-file-gguf-support.md) — the
  original feasibility study and Go/No-Go framework this feature was built
  under.
