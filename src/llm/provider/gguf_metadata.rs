//! Pure-Rust GGUF header/metadata parser — no FFI, no `llama-cpp-2` dependency.
//!
//! Reads only the GGUF header, metadata key-value section, and tensor-info
//! section of a `.gguf` file — never the (multi-GB) tensor data blob that
//! follows. See `ccguf-managment-imrpoment-plan.md` Phase M1 for the design
//! rationale and the public GGUF spec this implements against.
//!
//! **Hardening**: every value handed to this parser may come from an
//! untrusted source (a file downloaded from an arbitrary URL via
//! `llama_cpp_models::download_model`, or just a corrupted/truncated file).
//! Every declared length is validated against a cap *before* anything
//! allocation-shaped is called on it, and any structural problem (bad
//! magic, unsupported version, truncation, or a cap violation) makes the
//! whole parse return `None` rather than panicking or returning
//! partial/corrupt data. A structurally valid file that's simply missing an
//! optional key is not such a problem — it still returns `Some`, with
//! `None` in the specific missing fields.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// GGUF magic bytes ("GGUF"), compared byte-for-byte rather than as a
/// little/big-endian `u32` to sidestep any endianness ambiguity entirely.
const GGUF_MAGIC: [u8; 4] = *b"GGUF";

/// GGUF versions this parser understands. Older files are rare in practice
/// (llama.cpp has published v3 for a long time); newer major versions, if
/// they ever appear, are safer to decline than to assume compatibility with.
const MIN_SUPPORTED_VERSION: u32 = 2;
const MAX_SUPPORTED_VERSION: u32 = 3;

/// Per-value hardening caps — generous enough for any real GGUF file (a
/// tokenizer vocabulary array commonly runs to 100-200k entries, for
/// example) while still bounding worst-case adversarial memory use. See the
/// module doc comment and `ccguf-managment-imrpoment-plan.md` Phase M1's
/// "Hardening" section for the reasoning behind each number.
const MAX_KV_COUNT: u64 = 100_000;
const MAX_TENSOR_COUNT: u64 = 100_000;
const MAX_TENSOR_DIMS: u32 = 8;
const MAX_STRING_BYTES: u64 = 64 * 1024 * 1024;
const MAX_ARRAY_LEN: u64 = 1_000_000;
/// Cumulative cap on total bytes read while parsing the KV + tensor-info
/// sections, independent of the per-value caps above — defense in depth
/// against many small-but-not-individually-over-cap values adding up.
const MAX_TOTAL_METADATA_BYTES: u64 = 256 * 1024 * 1024;

/// Parsed GGUF metadata. Every field is best-effort/`Option` — see the
/// module doc comment for when `read_gguf_metadata` returns `None` for the
/// whole file versus `Some` with individual fields unset.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GgufMetadata {
    pub architecture: Option<String>,
    pub name: Option<String>,
    /// Total scalar weight count, summed across every tensor's dimensions —
    /// independent of quantization (which affects storage bytes per
    /// parameter, not the logical parameter count).
    pub parameter_count: Option<u64>,
    /// Best-effort quantization label. Prefers `general.file_type` (precise
    /// — distinguishes e.g. `Q4_K_S` from `Q4_K_M`) over the tensor-type
    /// mode (coarser — that distinction doesn't exist at the per-tensor
    /// level, only in the file-level preset name).
    pub quantization: Option<String>,
    pub context_length: Option<u64>,
    pub has_chat_template: bool,
    /// Number of transformer blocks/layers (`<arch>.block_count`). Used
    /// only by `estimate_memory_bytes`'s KV-cache term - not otherwise
    /// user-facing, so `LocalGgufModel` doesn't carry this or the three
    /// fields below directly, only the derived estimate.
    pub block_count: Option<u64>,
    /// `<arch>.attention.head_count` - the plain (non-KV) attention head
    /// count, used to derive per-head dimension.
    pub attention_head_count: Option<u64>,
    /// `<arch>.attention.head_count_kv` - distinct from
    /// `attention_head_count` on GQA/MQA architectures (fewer KV heads
    /// than query heads); `None` on architectures that don't set it
    /// separately, in which case `attention_head_count` is the right
    /// fallback (no GQA reduction).
    pub attention_head_count_kv: Option<u64>,
    /// `<arch>.embedding_length` - divided by `attention_head_count` to
    /// get the per-head dimension.
    pub embedding_length: Option<u64>,
    /// `clip.has_vision_encoder` - identifies a vision projector (mmproj)
    /// GGUF file, verified against llama.cpp's own `tools/mtmd/
    /// clip-impl.h` rather than assumed (not `general.architecture ==
    /// "clip"`, which doesn't exist as a detection signal in the actual
    /// source). `false` both when the key is absent and when it's present
    /// but `false` - both mean "not established as a vision projector."
    pub is_vision_projector: bool,
    /// `clip.has_audio_encoder` - the audio-projector equivalent of
    /// `is_vision_projector`, same source and same `false`-means-both
    /// semantics.
    pub is_audio_projector: bool,
}

/// Read `path`'s GGUF header/metadata. `None` on any structural problem —
/// not a valid GGUF file, truncated, or a value exceeds this parser's
/// hardening caps — never panics. Only the header/KV/tensor-info sections
/// are read; the tensor data blob itself (the bulk of the file) is never
/// touched, so this stays fast and memory-light even for a multi-GB model.
pub fn read_gguf_metadata(path: &Path) -> Option<GgufMetadata> {
    let file = File::open(path).ok()?;
    let mut reader = BoundedReader::new(BufReader::new(file));
    parse(&mut reader)
}

/// GGUF metadata value type tags (spec-defined, fixed numbering).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    Bool,
    String,
    Array,
    U64,
    I64,
    F64,
}

impl ValueType {
    fn from_tag(tag: u32) -> Option<Self> {
        Some(match tag {
            0 => Self::U8,
            1 => Self::I8,
            2 => Self::U16,
            3 => Self::I16,
            4 => Self::U32,
            5 => Self::I32,
            6 => Self::F32,
            7 => Self::Bool,
            8 => Self::String,
            9 => Self::Array,
            10 => Self::U64,
            11 => Self::I64,
            12 => Self::F64,
            _ => return None,
        })
    }

    /// Byte width of a scalar of this type. `None` for `String`/`Array`,
    /// which aren't fixed-width.
    fn scalar_width(self) -> Option<usize> {
        Some(match self {
            Self::U8 | Self::I8 | Self::Bool => 1,
            Self::U16 | Self::I16 => 2,
            Self::U32 | Self::I32 | Self::F32 => 4,
            Self::U64 | Self::I64 | Self::F64 => 8,
            Self::String | Self::Array => return None,
        })
    }
}

/// A decoded value, retained only for the types this parser actually cares
/// about (strings, and the unsigned-integer types used by
/// `general.file_type`/`*.context_length`). Every other type is still fully
/// consumed from the stream (so subsequent KV pairs stay in sync) but
/// discarded as `Other`.
enum DecodedValue {
    Str(String),
    UInt(u64),
    Bool(bool),
    Other,
}

/// Wraps a `Read` with the cumulative byte-budget enforcement shared by
/// every read in this module — a read fails once the budget is exhausted,
/// independent of any individual value's own cap.
struct BoundedReader<R> {
    inner: R,
    remaining_budget: u64,
}

impl<R: Read> BoundedReader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            remaining_budget: MAX_TOTAL_METADATA_BYTES,
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Option<()> {
        let len = buf.len() as u64;
        if len > self.remaining_budget {
            return None;
        }
        self.inner.read_exact(buf).ok()?;
        self.remaining_budget -= len;
        Some(())
    }

    fn read_u32(&mut self) -> Option<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Some(u32::from_le_bytes(buf))
    }

    fn read_u64(&mut self) -> Option<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Some(u64::from_le_bytes(buf))
    }

    /// Reads a length-prefixed GGUF string. The declared length is checked
    /// against `MAX_STRING_BYTES` *before* the backing buffer is allocated
    /// — the core hardening invariant of this module: never allocate from
    /// an unchecked, attacker-influenced length.
    fn read_string(&mut self) -> Option<String> {
        let len = self.read_u64()?;
        if len > MAX_STRING_BYTES {
            return None;
        }
        // Safe to allocate: `len` is now bounded by `MAX_STRING_BYTES` (64
        // MiB), not the raw declared value.
        let mut buf = vec![0u8; len as usize];
        self.read_exact(&mut buf)?;
        Some(String::from_utf8_lossy(&buf).into_owned())
    }

    /// Reads one value of `ty`, returning a decoded form for the types this
    /// parser retains (`String`, `U32`/`U64`) or `Other` for everything
    /// else. Always fully consumes the value's bytes from the stream
    /// regardless of whether it's retained, so the next KV pair or tensor
    /// info starts at the right offset.
    fn read_value(&mut self, ty: ValueType) -> Option<DecodedValue> {
        match ty {
            ValueType::String => Some(DecodedValue::Str(self.read_string()?)),
            ValueType::Array => {
                let elem_tag = self.read_u32()?;
                let elem_ty = ValueType::from_tag(elem_tag)?;
                let len = self.read_u64()?;
                if len > MAX_ARRAY_LEN {
                    return None;
                }
                // Arrays aren't retained by this parser (no field needs
                // array contents today) - each element is still fully
                // decoded to keep the stream position correct.
                for _ in 0..len {
                    self.read_value(elem_ty)?;
                }
                Some(DecodedValue::Other)
            }
            _ => {
                let width = ty
                    .scalar_width()
                    .expect("non-String/Array type has a scalar width");
                let mut buf = [0u8; 8];
                self.read_exact(&mut buf[..width])?;
                Some(match ty {
                    ValueType::U32 => {
                        DecodedValue::UInt(u32::from_le_bytes(buf[..4].try_into().ok()?) as u64)
                    }
                    ValueType::U64 => {
                        DecodedValue::UInt(u64::from_le_bytes(buf[..8].try_into().ok()?))
                    }
                    ValueType::Bool => DecodedValue::Bool(buf[0] != 0),
                    _ => DecodedValue::Other,
                })
            }
        }
    }
}

fn parse(reader: &mut BoundedReader<impl Read>) -> Option<GgufMetadata> {
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;
    if magic != GGUF_MAGIC {
        return None;
    }

    let version = reader.read_u32()?;
    if !(MIN_SUPPORTED_VERSION..=MAX_SUPPORTED_VERSION).contains(&version) {
        return None;
    }

    let tensor_count = reader.read_u64()?;
    if tensor_count > MAX_TENSOR_COUNT {
        return None;
    }
    let kv_count = reader.read_u64()?;
    if kv_count > MAX_KV_COUNT {
        return None;
    }

    let mut metadata = GgufMetadata::default();
    let mut file_type: Option<u64> = None;

    for _ in 0..kv_count {
        let key = reader.read_string()?;
        let value_tag = reader.read_u32()?;
        let value_ty = ValueType::from_tag(value_tag)?;
        let value = reader.read_value(value_ty)?;

        match (key.as_str(), value) {
            ("general.architecture", DecodedValue::Str(s)) => metadata.architecture = Some(s),
            ("general.name", DecodedValue::Str(s)) => metadata.name = Some(s),
            ("general.file_type", DecodedValue::UInt(n)) => file_type = Some(n),
            ("tokenizer.chat_template", _) => metadata.has_chat_template = true,
            (k, DecodedValue::UInt(n)) if k.ends_with(".context_length") => {
                metadata.context_length = Some(n);
            }
            (k, DecodedValue::UInt(n)) if k.ends_with(".block_count") => {
                metadata.block_count = Some(n);
            }
            // Checked before the plain "...head_count" arm below - a key
            // ending in "_kv" can never also match the non-"_kv" suffix,
            // but ordering these explicitly documents that intent rather
            // than relying on it silently.
            (k, DecodedValue::UInt(n)) if k.ends_with(".attention.head_count_kv") => {
                metadata.attention_head_count_kv = Some(n);
            }
            (k, DecodedValue::UInt(n)) if k.ends_with(".attention.head_count") => {
                metadata.attention_head_count = Some(n);
            }
            (k, DecodedValue::UInt(n)) if k.ends_with(".embedding_length") => {
                metadata.embedding_length = Some(n);
            }
            ("clip.has_vision_encoder", DecodedValue::Bool(b)) => {
                metadata.is_vision_projector = b;
            }
            ("clip.has_audio_encoder", DecodedValue::Bool(b)) => {
                metadata.is_audio_projector = b;
            }
            _ => {}
        }
    }

    // Tensor-info section: always parsed (cheap - no tensor *data* is read)
    // for parameter count and the quantization fallback.
    let mut param_count: Option<u64> = Some(0);
    let mut type_counts: HashMap<u32, u32> = HashMap::new();

    for _ in 0..tensor_count {
        let _name = reader.read_string()?;
        let n_dims = reader.read_u32()?;
        if n_dims > MAX_TENSOR_DIMS {
            return None;
        }

        let mut tensor_elems: Option<u64> = Some(1);
        for _ in 0..n_dims {
            let dim = reader.read_u64()?;
            tensor_elems = tensor_elems.and_then(|acc| acc.checked_mul(dim));
        }

        let tensor_type = reader.read_u32()?;
        let _offset = reader.read_u64()?;

        param_count = match (param_count, tensor_elems) {
            (Some(total), Some(elems)) => total.checked_add(elems),
            _ => None,
        };
        *type_counts.entry(tensor_type).or_insert(0) += 1;
    }

    metadata.parameter_count = param_count;
    metadata.quantization = file_type.and_then(ftype_to_string).or_else(|| {
        type_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .and_then(|(ty, _)| ggml_type_to_string(ty))
    });

    Some(metadata)
}

/// Maps `general.file_type` (the `ggml_ftype`/`llama_ftype` enum — a
/// file-level "which quantization preset was used" value, distinct from
/// the per-tensor `ggml_type` enum below) to its human name.
///
/// Deliberately incomplete: only entries verified with high confidence
/// against the public spec are mapped. `ggml_ftype` has grown many `IQ*`/
/// `TQ*` variants over time whose exact integer codes aren't worth
/// guessing at — an unmapped code has no entry here and the caller falls
/// through to the coarser tensor-type-mode inference (or ultimately
/// "unknown"), never a wrong name. Extend this table when a specific,
/// verified code is reported missing, not preemptively.
fn ftype_to_string(code: u64) -> Option<String> {
    let name = match code {
        0 => "F32",
        1 => "F16",
        2 => "Q4_0",
        3 => "Q4_1",
        7 => "Q8_0",
        8 => "Q5_0",
        9 => "Q5_1",
        10 => "Q2_K",
        11 => "Q3_K_S",
        12 => "Q3_K_M",
        13 => "Q3_K_L",
        14 => "Q4_K_S",
        15 => "Q4_K_M",
        16 => "Q5_K_S",
        17 => "Q5_K_M",
        18 => "Q6_K",
        _ => return None,
    };
    Some(name.to_string())
}

/// Maps a per-tensor `ggml_type` code to its human name. Coarser than
/// `ftype_to_string` above by nature — the K-quant `_S`/`_M`/`_L` preset
/// suffix is a file-level naming convention (which tensors got which
/// quantization under a given strategy), not a distinct raw tensor type,
/// so this table can only express the base family (e.g. `"Q4_K"`, not
/// `"Q4_K_M"`). Same "verified entries only" policy as `ftype_to_string`.
fn ggml_type_to_string(code: u32) -> Option<String> {
    let name = match code {
        0 => "F32",
        1 => "F16",
        2 => "Q4_0",
        3 => "Q4_1",
        6 => "Q5_0",
        7 => "Q5_1",
        8 => "Q8_0",
        9 => "Q8_1",
        10 => "Q2_K",
        11 => "Q3_K",
        12 => "Q4_K",
        13 => "Q5_K",
        14 => "Q6_K",
        15 => "Q8_K",
        _ => return None,
    };
    Some(name.to_string())
}

/// A resident-memory estimate for loading a model at a given context length.
/// See `estimate_memory_bytes`'s doc comment for what's included and the
/// accuracy this is meant to convey (order-of-magnitude, not exact).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryEstimate {
    pub total_bytes: u64,
    /// `false` when the header didn't have enough attention geometry
    /// (`block_count`/`attention.head_count[_kv]`/`embedding_length`) to
    /// compute the KV-cache term - `total_bytes` then covers weights only,
    /// and callers should say so rather than presenting it as complete.
    pub includes_kv_cache: bool,
}

/// Approximate bits-per-weight for quantization labels this module's own
/// lookup tables (`ftype_to_string`, `ggml_type_to_string`) and
/// `quantization_hint_from_filename`'s filename-convention fallback can
/// produce. These are published, widely-cited community figures from the
/// llama.cpp/GGUF ecosystem, not exact - real bpw varies slightly by the
/// specific per-tensor mix within a "mostly X" preset. Good enough for an
/// order-of-magnitude estimate, which is all `estimate_memory_bytes` claims
/// to give; `None` for a label this table doesn't cover degrades to "can't
/// estimate," not a guess.
fn bits_per_weight(quantization: &str) -> Option<f64> {
    Some(match quantization {
        "F32" => 32.0,
        "F16" | "BF16" => 16.0,
        "Q8_0" | "Q8_1" | "Q8_K" => 8.5,
        "Q6_K" => 6.6,
        "Q5_0" | "Q5_1" => 5.5,
        "Q5_K" | "Q5_K_S" => 5.5,
        "Q5_K_M" => 5.7,
        "Q4_0" | "Q4_1" => 4.5,
        "Q4_K" | "Q4_K_S" => 4.5,
        "Q4_K_M" => 4.85,
        "IQ4_NL" => 4.5,
        "IQ4_XS" => 4.25,
        "Q3_K" | "Q3_K_S" => 3.5,
        "Q3_K_M" => 3.75,
        "Q3_K_L" => 4.1,
        "IQ3_M" => 3.66,
        "IQ3_S" | "IQ3_XXS" => 3.44,
        "Q2_K" => 2.56,
        "IQ2_M" => 2.7,
        "IQ2_S" => 2.5,
        "IQ2_XS" => 2.31,
        "IQ2_XXS" => 2.06,
        _ => return None,
    })
}

/// Estimate resident memory (weights + KV cache, when the header has
/// enough geometry to compute it) for loading `metadata` at
/// `context_length` tokens. `None` - not a fallback guess - when
/// `parameter_count`/`quantization` themselves are unavailable, same
/// "honest unknown" posture as the rest of this module.
pub fn estimate_memory_bytes(
    metadata: &GgufMetadata,
    context_length: u64,
) -> Option<MemoryEstimate> {
    let params = metadata.parameter_count?;
    let bpw = bits_per_weight(metadata.quantization.as_deref()?)?;
    let weight_bytes = (params as f64 * bpw / 8.0) as u64;

    let kv_bytes = (|| -> Option<u64> {
        let blocks = metadata.block_count?;
        let head_count = metadata.attention_head_count?;
        let heads_kv = metadata.attention_head_count_kv.unwrap_or(head_count);
        let embedding_length = metadata.embedding_length?;
        if head_count == 0 {
            return None;
        }
        let head_dim = embedding_length / head_count;
        // 2 (K and V) * blocks * kv-heads * head_dim * 2 bytes/element
        // (f16 KV cache) * context_length.
        2u64.checked_mul(blocks)?
            .checked_mul(heads_kv)?
            .checked_mul(head_dim)?
            .checked_mul(2)?
            .checked_mul(context_length)
    })();

    Some(match kv_bytes {
        Some(kv) => MemoryEstimate {
            total_bytes: weight_bytes.saturating_add(kv),
            includes_kv_cache: true,
        },
        None => MemoryEstimate {
            total_bytes: weight_bytes,
            includes_kv_cache: false,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Appends a GGUF length-prefixed string.
    fn push_string(buf: &mut Vec<u8>, s: &str) {
        buf.extend_from_slice(&(s.len() as u64).to_le_bytes());
        buf.extend_from_slice(s.as_bytes());
    }

    /// Appends one `general.architecture`/`general.name`-shaped STRING KV
    /// pair (`key`, value type tag 8, then the length-prefixed value).
    fn push_string_kv(buf: &mut Vec<u8>, key: &str, value: &str) {
        push_string(buf, key);
        buf.extend_from_slice(&8u32.to_le_bytes()); // ValueType::String
        push_string(buf, value);
    }

    /// Appends one U32 KV pair (used for `general.file_type` and
    /// `*.context_length` in these tests).
    fn push_u32_kv(buf: &mut Vec<u8>, key: &str, value: u32) {
        push_string(buf, key);
        buf.extend_from_slice(&4u32.to_le_bytes()); // ValueType::U32
        buf.extend_from_slice(&value.to_le_bytes());
    }

    fn push_bool_kv(buf: &mut Vec<u8>, key: &str, value: bool) {
        push_string(buf, key);
        buf.extend_from_slice(&7u32.to_le_bytes()); // ValueType::Bool
        buf.push(value as u8);
    }

    /// Appends one tensor-info entry: name, dims, `ggml_type`, offset.
    fn push_tensor(buf: &mut Vec<u8>, name: &str, dims: &[u64], ggml_type: u32) {
        push_string(buf, name);
        buf.extend_from_slice(&(dims.len() as u32).to_le_bytes());
        for d in dims {
            buf.extend_from_slice(&d.to_le_bytes());
        }
        buf.extend_from_slice(&ggml_type.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes()); // offset - unused by this parser
    }

    fn header(tensor_count: u64, kv_count: u64) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"GGUF");
        buf.extend_from_slice(&3u32.to_le_bytes()); // version
        buf.extend_from_slice(&tensor_count.to_le_bytes());
        buf.extend_from_slice(&kv_count.to_le_bytes());
        buf
    }

    fn write_temp_gguf(bytes: &[u8]) -> tempfile::NamedTempFile {
        use std::io::Write as _;
        let mut file = tempfile::NamedTempFile::new().expect("create temp file");
        file.write_all(bytes).expect("write temp file");
        file
    }

    #[test]
    fn parses_architecture_name_context_length_and_chat_template() {
        let mut buf = header(0, 4);
        push_string_kv(&mut buf, "general.architecture", "qwen2");
        push_string_kv(&mut buf, "general.name", "Test Model");
        push_u32_kv(&mut buf, "qwen2.context_length", 32768);
        push_string_kv(&mut buf, "tokenizer.chat_template", "{{ messages }}");

        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path()).expect("must parse a well-formed file");

        assert_eq!(metadata.architecture.as_deref(), Some("qwen2"));
        assert_eq!(metadata.name.as_deref(), Some("Test Model"));
        assert_eq!(metadata.context_length, Some(32768));
        assert!(metadata.has_chat_template);
        assert_eq!(
            metadata.parameter_count,
            Some(0),
            "zero tensors, zero params"
        );
    }

    #[test]
    fn missing_optional_keys_still_parses_successfully() {
        let buf = header(0, 0);
        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path())
            .expect("a structurally valid file with no KV pairs is not a parse failure");

        assert_eq!(metadata.architecture, None);
        assert!(!metadata.has_chat_template);
        assert_eq!(metadata.quantization, None);
    }

    #[test]
    fn clip_has_vision_encoder_true_sets_is_vision_projector() {
        let mut buf = header(0, 1);
        push_bool_kv(&mut buf, "clip.has_vision_encoder", true);

        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path()).expect("must parse");
        assert!(metadata.is_vision_projector);
        assert!(!metadata.is_audio_projector);
    }

    #[test]
    fn clip_has_vision_encoder_false_does_not_set_is_vision_projector() {
        let mut buf = header(0, 1);
        push_bool_kv(&mut buf, "clip.has_vision_encoder", false);

        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path()).expect("must parse");
        assert!(!metadata.is_vision_projector);
    }

    #[test]
    fn clip_has_audio_encoder_true_sets_is_audio_projector() {
        let mut buf = header(0, 1);
        push_bool_kv(&mut buf, "clip.has_audio_encoder", true);

        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path()).expect("must parse");
        assert!(metadata.is_audio_projector);
        assert!(!metadata.is_vision_projector);
    }

    #[test]
    fn neither_clip_key_present_leaves_both_projector_flags_false() {
        let buf = header(0, 0);
        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path()).expect("must parse");
        assert!(!metadata.is_vision_projector);
        assert!(!metadata.is_audio_projector);
    }

    #[test]
    fn general_file_type_maps_to_the_precise_quantization_string() {
        let mut buf = header(0, 1);
        push_u32_kv(&mut buf, "general.file_type", 15); // MOSTLY_Q4_K_M

        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path()).expect("must parse");
        assert_eq!(metadata.quantization.as_deref(), Some("Q4_K_M"));
    }

    #[test]
    fn tensor_type_mode_is_the_fallback_when_file_type_is_absent() {
        let mut buf = header(4, 0);
        // Three Q4_K tensors, one F32 (e.g. a token embedding kept at
        // higher precision) - Q4_K is the mode.
        push_tensor(&mut buf, "blk.0.weight", &[4096, 4096], 12); // GGML_TYPE_Q4_K
        push_tensor(&mut buf, "blk.1.weight", &[4096, 4096], 12);
        push_tensor(&mut buf, "blk.2.weight", &[4096, 4096], 12);
        push_tensor(&mut buf, "token_embd.weight", &[4096, 32000], 0); // GGML_TYPE_F32

        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path()).expect("must parse");
        assert_eq!(
            metadata.quantization.as_deref(),
            Some("Q4_K"),
            "the coarser per-tensor mode can't express the _M/_S/_L preset suffix"
        );
    }

    #[test]
    fn parameter_count_sums_across_tensor_dimensions() {
        let mut buf = header(2, 0);
        push_tensor(&mut buf, "a", &[4096, 4096], 0); // 16,777,216
        push_tensor(&mut buf, "b", &[128, 256, 2], 0); // 65,536

        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path()).expect("must parse");
        assert_eq!(metadata.parameter_count, Some(4096 * 4096 + 128 * 256 * 2));
    }

    #[test]
    fn a_string_array_between_two_kv_pairs_is_skipped_correctly() {
        let mut buf = header(0, 3);
        push_string_kv(&mut buf, "general.architecture", "qwen2");
        // tokenizer.ggml.tokens-shaped STRING array - must be fully (and
        // correctly) skipped for `general.name` below to still parse.
        push_string(&mut buf, "tokenizer.ggml.tokens");
        buf.extend_from_slice(&9u32.to_le_bytes()); // ValueType::Array
        buf.extend_from_slice(&8u32.to_le_bytes()); // element type: String
        buf.extend_from_slice(&3u64.to_le_bytes()); // 3 elements
        push_string(&mut buf, "<s>");
        push_string(&mut buf, "</s>");
        push_string(&mut buf, "hello");
        push_string_kv(&mut buf, "general.name", "After The Array");

        let file = write_temp_gguf(&buf);
        let metadata = read_gguf_metadata(file.path()).expect("must parse");
        assert_eq!(metadata.architecture.as_deref(), Some("qwen2"));
        assert_eq!(metadata.name.as_deref(), Some("After The Array"));
    }

    #[test]
    fn rejects_bad_magic() {
        let mut buf = b"NOPE".to_vec();
        buf.extend_from_slice(&3u32.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());

        let file = write_temp_gguf(&buf);
        assert!(read_gguf_metadata(file.path()).is_none());
    }

    #[test]
    fn rejects_unsupported_version() {
        let mut buf = b"GGUF".to_vec();
        buf.extend_from_slice(&99u32.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());

        let file = write_temp_gguf(&buf);
        assert!(read_gguf_metadata(file.path()).is_none());
    }

    #[test]
    fn truncation_at_several_points_returns_none_not_a_panic() {
        let mut full = header(0, 1);
        push_string_kv(&mut full, "general.architecture", "qwen2");

        // Cut at every byte offset from 0 up to (but not including) the
        // full length - none of them should panic, all should report a
        // clean parse failure.
        for cut in 0..full.len() {
            let file = write_temp_gguf(&full[..cut]);
            assert!(
                read_gguf_metadata(file.path()).is_none(),
                "truncation at byte {cut} of {} should fail cleanly",
                full.len()
            );
        }
    }

    #[test]
    fn oversized_declared_string_length_is_rejected_before_allocating() {
        // A KV pair claiming a ~1 TiB string, with no actual bytes behind
        // it. If the cap were checked *after* attempting to allocate, this
        // would abort or hang the test process; completing quickly with
        // `None` is the proof the length was validated first.
        let mut buf = header(0, 1);
        push_string(&mut buf, "general.architecture");
        buf.extend_from_slice(&8u32.to_le_bytes()); // ValueType::String
        buf.extend_from_slice(&(1u64 << 40).to_le_bytes()); // declared length, no data follows

        let file = write_temp_gguf(&buf);
        assert!(read_gguf_metadata(file.path()).is_none());
    }

    #[test]
    fn oversized_declared_array_length_is_rejected() {
        let mut buf = header(0, 1);
        push_string(&mut buf, "tokenizer.ggml.tokens");
        buf.extend_from_slice(&9u32.to_le_bytes()); // ValueType::Array
        buf.extend_from_slice(&8u32.to_le_bytes()); // element type: String
        buf.extend_from_slice(&(MAX_ARRAY_LEN + 1).to_le_bytes()); // over the cap, no elements follow

        let file = write_temp_gguf(&buf);
        assert!(read_gguf_metadata(file.path()).is_none());
    }

    #[test]
    fn oversized_declared_kv_count_is_rejected_before_reading_any_of_them() {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"GGUF");
        buf.extend_from_slice(&3u32.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes()); // tensor_count
        buf.extend_from_slice(&(MAX_KV_COUNT + 1).to_le_bytes()); // kv_count - no KV data follows

        let file = write_temp_gguf(&buf);
        assert!(read_gguf_metadata(file.path()).is_none());
    }

    #[test]
    fn nonexistent_file_returns_none() {
        let path = std::path::Path::new("/definitely/does/not/exist/crustly-gguf-test.gguf");
        assert!(read_gguf_metadata(path).is_none());
    }

    #[test]
    fn estimate_memory_bytes_is_none_without_parameter_count_or_quantization() {
        let missing_params = GgufMetadata {
            quantization: Some("Q4_K_M".to_string()),
            ..Default::default()
        };
        assert!(estimate_memory_bytes(&missing_params, 8192).is_none());

        let missing_quant = GgufMetadata {
            parameter_count: Some(7_000_000_000),
            ..Default::default()
        };
        assert!(estimate_memory_bytes(&missing_quant, 8192).is_none());
    }

    #[test]
    fn estimate_memory_bytes_covers_weights_only_when_geometry_is_missing() {
        let metadata = GgufMetadata {
            parameter_count: Some(7_000_000_000),
            quantization: Some("Q4_K_M".to_string()),
            ..Default::default()
        };
        let estimate = estimate_memory_bytes(&metadata, 8192).expect("must estimate");
        assert!(!estimate.includes_kv_cache);
        // ~7B params at ~4.85 bits/weight -> roughly 4.2 GB of weights alone.
        assert!(estimate.total_bytes > 3_500_000_000 && estimate.total_bytes < 5_000_000_000);
    }

    #[test]
    fn estimate_memory_bytes_includes_kv_cache_when_geometry_is_present() {
        let metadata = GgufMetadata {
            parameter_count: Some(7_000_000_000),
            quantization: Some("Q4_K_M".to_string()),
            block_count: Some(32),
            attention_head_count: Some(32),
            attention_head_count_kv: Some(8), // GQA - fewer KV heads than query heads
            embedding_length: Some(4096),
            ..Default::default()
        };
        let weights_only = estimate_memory_bytes(
            &GgufMetadata {
                parameter_count: metadata.parameter_count,
                quantization: metadata.quantization.clone(),
                ..Default::default()
            },
            8192,
        )
        .expect("weights-only estimate");
        let with_kv = estimate_memory_bytes(&metadata, 8192).expect("must estimate");

        assert!(with_kv.includes_kv_cache);
        assert!(
            with_kv.total_bytes > weights_only.total_bytes,
            "the KV-cache term should add to the weights-only baseline"
        );
    }

    #[test]
    fn estimate_memory_bytes_falls_back_to_head_count_without_gqa_kv_heads() {
        let metadata = GgufMetadata {
            parameter_count: Some(1_000_000_000),
            quantization: Some("F16".to_string()),
            block_count: Some(16),
            attention_head_count: Some(16),
            attention_head_count_kv: None, // no GQA - falls back to attention_head_count
            embedding_length: Some(2048),
            ..Default::default()
        };
        let estimate = estimate_memory_bytes(&metadata, 4096).expect("must estimate");
        assert!(estimate.includes_kv_cache);
    }
}
