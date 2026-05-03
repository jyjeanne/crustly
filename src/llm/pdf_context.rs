//! PDF Context Injection
//!
//! Implements the Claw Code pattern of auto-detecting PDF path references in user messages
//! and prepending extracted text before the message reaches the LLM.
//!
//! This saves the model a tool-call round-trip: the user can write
//! "summarise report.pdf" and the extracted text is already in context.
//!
//! Unlike Claw Code's hand-rolled byte scanner, Crustly uses the `pdf-extract`
//! crate (already a dependency) which handles a wider range of PDFs.

use std::path::{Path, PathBuf};

/// Maximum characters of PDF content to inject into a single message.
/// Larger PDFs are truncated with a notice; the user can use `parse_document`
/// with `max_chars` / `pages` options for finer-grained access.
const MAX_INJECTED_PDF_CHARS: usize = 40_000;

/// Scan `text` for a token that looks like a PDF file path.
///
/// Accepts both absolute paths and bare filenames.  Quotes, backticks, and
/// single-quotes around the path are stripped.  Comparison is case-insensitive
/// so `.PDF` is recognised as well as `.pdf`.
///
/// Returns the first match resolved against `cwd` if relative, or as-is if
/// absolute.  Returns `None` if no `.pdf` token is found or the resolved path
/// does not exist on disk.
pub fn looks_like_pdf_path(text: &str, cwd: &Path) -> Option<PathBuf> {
    for token in text.split_whitespace() {
        let cleaned = token.trim_matches(|c: char| matches!(c, '\'' | '"' | '`' | ',' | ';'));
        if let Some(dot_pos) = cleaned.rfind('.') {
            if dot_pos == 0 {
                continue; // hidden file starting with "."
            }
            if cleaned[dot_pos + 1..].eq_ignore_ascii_case("pdf") {
                let path = PathBuf::from(cleaned);
                let resolved = if path.is_absolute() {
                    path
                } else {
                    cwd.join(path)
                };
                if resolved.exists() && resolved.is_file() {
                    return Some(resolved);
                }
            }
        }
    }
    None
}

/// Extract text from a PDF file using the `pdf-extract` crate.
///
/// Run inside `spawn_blocking` by the caller when an async context is active.
/// Returns `Err` with a human-readable message on failure.
pub fn extract_pdf_text(path: &Path) -> Result<String, String> {
    let bytes =
        std::fs::read(path).map_err(|e| format!("failed to read '{}': {}", path.display(), e))?;
    pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| format!("PDF extraction failed for '{}': {}", path.display(), e))
}

/// If `message` contains a reference to an existing PDF file, extract its text
/// and return a new message with the PDF content prepended.
///
/// Format of the augmented message:
/// ```text
/// [PDF Content: path/to/file.pdf]
/// <extracted text, possibly truncated>
/// [End of PDF Content]
///
/// <original user message>
/// ```
///
/// Returns the original message unchanged when:
/// - no `.pdf` path token is found
/// - the file does not exist on disk
/// - extraction fails (logged as a warning)
pub async fn augment_message_with_pdf(message: &str, cwd: &Path) -> String {
    let Some(pdf_path) = looks_like_pdf_path(message, cwd) else {
        return message.to_string();
    };

    let path_clone = pdf_path.clone();
    let extraction = tokio::task::spawn_blocking(move || extract_pdf_text(&path_clone)).await;

    let text = match extraction {
        Ok(Ok(t)) if !t.trim().is_empty() => t,
        Ok(Ok(_)) => {
            tracing::debug!(
                ?pdf_path,
                "PDF contained no extractable text; skipping injection"
            );
            return message.to_string();
        }
        Ok(Err(e)) => {
            tracing::warn!(?pdf_path, error = %e, "PDF extraction failed; skipping injection");
            return message.to_string();
        }
        Err(e) => {
            tracing::warn!(?pdf_path, error = %e, "PDF extraction task panicked; skipping injection");
            return message.to_string();
        }
    };

    // Find a safe byte index that does not split a multi-byte UTF-8 character.
    let safe_end = if text.len() > MAX_INJECTED_PDF_CHARS {
        (0..=MAX_INJECTED_PDF_CHARS)
            .rev()
            .find(|&i| text.is_char_boundary(i))
            .unwrap_or(0)
    } else {
        text.len()
    };
    let truncated = safe_end < text.len();
    let body = &text[..safe_end];

    let mut out = format!("[PDF Content: {}]\n{}\n", pdf_path.display(), body.trim());
    if truncated {
        out.push_str(&format!(
            "\n[PDF truncated: {} of {} characters shown — use the `parse_document` tool with `pages` or `max_chars` for more]\n",
            MAX_INJECTED_PDF_CHARS,
            text.len()
        ));
    }
    out.push_str("[End of PDF Content]\n\n");
    out.push_str(message);

    tracing::info!(
        path = %pdf_path.display(),
        chars = body.len(),
        truncated,
        "Auto-injected PDF content into user message"
    );

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn detects_absolute_pdf_token() {
        let dir = std::env::temp_dir();
        let pdf = dir.join("crustly_test_detect.pdf");
        std::fs::write(&pdf, b"%PDF-1.4").unwrap();

        let text = format!("Summarize {}", pdf.display());
        let result = looks_like_pdf_path(&text, &dir);
        assert_eq!(result, Some(pdf.clone()));
        let _ = std::fs::remove_file(pdf);
    }

    #[test]
    fn detects_relative_pdf_token() {
        let dir = std::env::temp_dir();
        let pdf = dir.join("crustly_test_rel.pdf");
        std::fs::write(&pdf, b"%PDF-1.4").unwrap();

        let text = "Summarize crustly_test_rel.pdf please";
        let result = looks_like_pdf_path(text, &dir);
        assert_eq!(result, Some(pdf.clone()));
        let _ = std::fs::remove_file(pdf);
    }

    #[test]
    fn case_insensitive_extension() {
        let dir = std::env::temp_dir();
        let pdf = dir.join("crustly_test_upper.PDF");
        std::fs::write(&pdf, b"%PDF-1.4").unwrap();

        let text = format!("Read {}", pdf.display());
        assert!(looks_like_pdf_path(&text, &dir).is_some());
        let _ = std::fs::remove_file(pdf);
    }

    #[test]
    fn strips_surrounding_quotes() {
        let dir = std::env::temp_dir();
        let pdf = dir.join("crustly_test_quoted.pdf");
        std::fs::write(&pdf, b"%PDF-1.4").unwrap();

        let text = format!("Read \"{}\" and summarize", pdf.display());
        assert!(looks_like_pdf_path(&text, &dir).is_some());
        let _ = std::fs::remove_file(pdf);
    }

    #[test]
    fn returns_none_for_missing_file() {
        let result =
            looks_like_pdf_path("Please read /nonexistent/path/xyz123.pdf", Path::new("/"));
        assert!(result.is_none());
    }

    #[test]
    fn returns_none_when_no_pdf() {
        let result = looks_like_pdf_path("Just a normal message with no pdf", Path::new("/"));
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn augment_returns_original_when_no_pdf() {
        let msg = "What is Rust?";
        let result = augment_message_with_pdf(msg, &std::env::temp_dir()).await;
        assert_eq!(result, msg);
    }

    #[tokio::test]
    async fn augment_returns_original_on_extraction_failure() {
        // Write a file that is NOT a valid PDF
        let mut f = NamedTempFile::with_suffix(".pdf").unwrap();
        writeln!(f, "not a real pdf").unwrap();
        f.flush().unwrap();
        let path = f.path().to_path_buf();
        let dir = path.parent().unwrap().to_path_buf();
        let filename = path.file_name().unwrap().to_string_lossy().into_owned();
        let msg = format!("Summarize {}", filename);
        let result = augment_message_with_pdf(&msg, &dir).await;
        // Should return unchanged because extraction yields empty/error
        assert_eq!(result, msg);
    }
}
