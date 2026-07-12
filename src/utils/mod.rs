//! Utility modules for common functionality

pub mod retry;

pub use retry::{retry, retry_with_check, RetryConfig, RetryableError};

/// Truncate `s` to at most `max_bytes` bytes without splitting a UTF-8
/// character. Direct byte slicing (`&s[..n]`) panics when `n` falls inside
/// a multi-byte character; this always returns a valid prefix.
pub fn truncate_at_char_boundary(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

#[cfg(test)]
mod tests {
    use super::truncate_at_char_boundary;

    #[test]
    fn truncate_ascii_at_exact_boundary() {
        assert_eq!(truncate_at_char_boundary("hello", 3), "hel");
        assert_eq!(truncate_at_char_boundary("hello", 10), "hello");
    }

    #[test]
    fn truncate_multibyte_does_not_panic() {
        // 'é' is 2 bytes; cutting at byte 1 must back off to the boundary.
        assert_eq!(truncate_at_char_boundary("héllo", 2), "h");
        // emoji is 4 bytes
        assert_eq!(truncate_at_char_boundary("🦀rust", 3), "");
        assert_eq!(truncate_at_char_boundary("🦀rust", 4), "🦀");
    }

    #[test]
    fn truncate_empty_and_zero() {
        assert_eq!(truncate_at_char_boundary("", 5), "");
        assert_eq!(truncate_at_char_boundary("abc", 0), "");
    }
}
