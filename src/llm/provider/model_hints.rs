//! Shared heuristics for guessing model capabilities from their name.
//!
//! Local model servers (Ollama, LM Studio, LocalAI, ...) don't expose a
//! structured "supports vision" flag over their OpenAI-compatible or native
//! chat APIs, so providers fall back to matching well-known substrings in
//! the model name/tag (e.g. `llava:13b`, `llama3.2-vision:11b-instruct-fp16`).

/// Returns `true` if `model_name` looks like a vision-capable model, based on
/// a case-insensitive substring match against known model families.
pub fn is_vision_model(model_name: &str) -> bool {
    let model_lc = model_name.to_lowercase();
    const VISION_PATTERNS: &[&str] = &[
        "llava",
        "vision",
        "minicpm-v",
        "bakllava",
        "moondream",
        "cogvlm",
        "qwen-vl",
        "qwenvl",
        "internvl",
        "phi-3-vision",
        "phi3-vision",
        "idefics",
    ];
    VISION_PATTERNS.iter().any(|p| model_lc.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_known_vision_models() {
        assert!(is_vision_model("llava:13b"));
        assert!(is_vision_model("llama3.2-vision:11b-instruct-fp16"));
        assert!(is_vision_model("MiniCPM-V-2.6"));
    }

    #[test]
    fn rejects_non_vision_models() {
        assert!(!is_vision_model("llama3.2:8b"));
        assert!(!is_vision_model(""));
    }
}
