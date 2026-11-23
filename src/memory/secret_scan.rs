//! Deterministic secret scanning for everything that is about to be stored
//! (docs/improve-memory.md §14.1). Never the only filter: a match rejects the
//! whole memory, and only the rule's name is ever reported, never the text.
//!
//! Test fixtures for these patterns are assembled at runtime, so a literal
//! key-shaped string does not appear in the source (GitHub push protection).

use regex::Regex;
use std::sync::LazyLock;

/// (rule name, pattern). Order does not matter: the first match wins.
const RULES: &[(&str, &str)] = &[
    ("private key block", r"-----BEGIN [A-Z ]*PRIVATE KEY-----"),
    ("AWS access key", r"AKIA[0-9A-Z]{16}"),
    ("GitHub token", r"gh[pousr]_[A-Za-z0-9]{36,}"),
    ("GitHub fine-grained token", r"github_pat_[A-Za-z0-9_]{50,}"),
    ("Anthropic API key", r"sk-ant-[A-Za-z0-9_-]{20,}"),
    ("OpenAI-style API key", r"sk-(proj-)?[A-Za-z0-9_-]{20,}"),
    ("Slack token", r"xox[abprs]-[A-Za-z0-9-]{10,}"),
    ("Google API key", r"AIza[0-9A-Za-z_-]{35}"),
    (
        "JSON Web Token",
        r"eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+",
    ),
    (
        "credentials in a URL",
        r"[a-z][a-z0-9+.-]*://[^/\s:@]+:[^/\s@]+@",
    ),
    (
        "password or key assignment",
        r"(?i)\b(password|passwd|pwd|secret|token|api[_-]?key|access[_-]?key)\b\s*[:=]\s*\S{6,}",
    ),
    (
        "authorization header",
        r"(?i)authorization:\s*(bearer|basic)\s+\S+",
    ),
    ("cookie header", r"(?i)\bcookie:\s*\S{8,}"),
];

/// A run of characters that can make up a key, checked by entropy below.
const TOKEN: &str = r"[A-Za-z0-9+/_=-]{32,}";

/// Above this many bits per character a token looks random. Hex digests top
/// out at 4.0, so commit hashes and UUIDs stay under it.
const ENTROPY_THRESHOLD: f64 = 4.5;

static COMPILED: LazyLock<Vec<(&'static str, Regex)>> = LazyLock::new(|| {
    RULES
        .iter()
        .map(|(name, pattern)| (*name, Regex::new(pattern).expect("valid secret rule")))
        .collect()
});

static TOKEN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(TOKEN).expect("valid token rule"));

/// Name of the first secret rule that `text` matches, or `None` when the
/// text looks clean.
pub fn scan(text: &str) -> Option<&'static str> {
    for (name, regex) in COMPILED.iter() {
        if regex.is_match(text) {
            return Some(name);
        }
    }
    if TOKEN_RE
        .find_iter(text)
        .any(|token| shannon_entropy(token.as_str()) > ENTROPY_THRESHOLD)
    {
        return Some("high-entropy token");
    }
    None
}

/// Bits per character of `s`, over the characters it contains.
fn shannon_entropy(s: &str) -> f64 {
    let mut counts = [0usize; 256];
    let mut total = 0usize;
    for byte in s.bytes() {
        counts[byte as usize] += 1;
        total += 1;
    }
    if total == 0 {
        return 0.0;
    }
    let total = total as f64;
    counts
        .iter()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let p = count as f64 / total;
            -p * p.log2()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a fake secret at runtime so no key-shaped literal is in the source.
    fn fake(parts: &[&str]) -> String {
        parts.concat()
    }

    #[test]
    fn detects_a_private_key_header() {
        let text = fake(&["-----BEGIN RSA ", "PRIVATE KEY-----"]);
        assert_eq!(scan(&text), Some("private key block"));
    }

    #[test]
    fn detects_an_aws_access_key() {
        let text = fake(&["key is AKIA", "ABCDEFGHIJKLMNOP"]);
        assert_eq!(scan(&text), Some("AWS access key"));
    }

    #[test]
    fn detects_github_tokens() {
        let classic = fake(&["ghp", "_", &"a1".repeat(18)]);
        assert_eq!(scan(&classic), Some("GitHub token"));
        let fine = fake(&["github", "_pat_", &"Ab_1".repeat(13)]);
        assert_eq!(scan(&fine), Some("GitHub fine-grained token"));
    }

    #[test]
    fn detects_provider_api_keys() {
        let anthropic = fake(&["sk", "-ant-", "api03-ABCDEFGHIJKLMNOPQRST"]);
        assert_eq!(scan(&anthropic), Some("Anthropic API key"));
        let openai = fake(&["sk", "-proj-", "ABCDEFGHIJKLMNOPQRSTUV"]);
        assert_eq!(scan(&openai), Some("OpenAI-style API key"));
    }

    #[test]
    fn detects_slack_and_google_keys() {
        let slack = fake(&["xox", "b-", "1234567890-abcdefghij"]);
        assert_eq!(scan(&slack), Some("Slack token"));
        let google = fake(&["AI", "za", "SyA1234567890abcdefghijklmnopqrstuv"]);
        assert_eq!(scan(&google), Some("Google API key"));
    }

    #[test]
    fn detects_a_jwt() {
        let jwt = fake(&[
            "eyJhbGciOiJIUzI1NiJ9",
            ".",
            "eyJzdWIiOiIxIn0",
            ".",
            "sig_part-1",
        ]);
        assert_eq!(scan(&jwt), Some("JSON Web Token"));
    }

    #[test]
    fn detects_credentials_in_a_url() {
        let url = fake(&["postgres://app:hunter2pw", "@db.internal/crustly"]);
        assert_eq!(scan(&url), Some("credentials in a URL"));
    }

    #[test]
    fn detects_password_assignments() {
        assert_eq!(
            scan("password = correct-horse"),
            Some("password or key assignment")
        );
        assert_eq!(
            scan("API_KEY: abcdefgh"),
            Some("password or key assignment")
        );
    }

    #[test]
    fn detects_authorization_and_cookie_headers() {
        let auth = fake(&["Authorization: Bear", "er abc.def.ghi"]);
        assert_eq!(scan(&auth), Some("authorization header"));
        assert_eq!(scan("Cookie: session=abcdef12"), Some("cookie header"));
    }

    #[test]
    fn detects_a_high_entropy_token() {
        // 40 distinct-looking characters: well above the threshold.
        let token = "Q7vR2xKp9LmZ4wT8yB1nC6hJ3fD5sG0aE2uI7oP9";
        assert_eq!(scan(&format!("value {token}")), Some("high-entropy token"));
    }

    #[test]
    fn ordinary_text_passes() {
        assert_eq!(scan("The user prefers Tokio for asynchronous Rust."), None);
        assert_eq!(scan("Run cargo clippy before committing."), None);
    }

    #[test]
    fn commit_hashes_and_uuids_pass() {
        // A 40-character git commit and a UUID are not secrets.
        assert_eq!(
            scan("fixed in 7e41a32cb1d2f0a9c8e7d6b5a4938271605f4e3d"),
            None
        );
        assert_eq!(scan("session 550e8400-e29b-41d4-a716-446655440000"), None);
    }

    #[test]
    fn only_the_rule_name_is_returned() {
        let secret = fake(&["AKIA", "ABCDEFGHIJKLMNOP"]);
        let rule = scan(&secret).unwrap();
        assert!(!rule.contains("AKIA"));
    }
}
