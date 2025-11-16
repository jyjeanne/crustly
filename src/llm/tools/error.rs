//! Tool error types

use thiserror::Error;

/// Tool error types
#[derive(Debug, Error)]
pub enum ToolError {
    /// Tool not found
    #[error("Tool not found: {0}")]
    NotFound(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Execution error
    #[error("Execution error: {0}")]
    Execution(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Approval required
    #[error("Tool requires approval: {0}")]
    ApprovalRequired(String),

    /// File not found
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Timeout
    #[error("Tool execution timed out after {0}s")]
    Timeout(u64),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for tool operations
pub type Result<T> = std::result::Result<T, ToolError>;

/// Validate that a path is safe and within the working directory
///
/// This function prevents path traversal attacks by:
/// 1. Resolving the path relative to working directory
/// 2. Canonicalizing both paths to resolve symlinks and ".."
/// 3. Ensuring the resolved path is within the working directory
pub fn validate_path_safety(
    requested_path: &str,
    working_directory: &std::path::Path,
) -> Result<std::path::PathBuf> {
    use std::path::PathBuf;

    // Resolve path relative to working directory
    let path = if PathBuf::from(requested_path).is_absolute() {
        PathBuf::from(requested_path)
    } else {
        working_directory.join(requested_path)
    };

    // For non-existent files, we need to check the parent directory
    // This handles cases where we're creating new files
    let path_to_check = if path.exists() {
        path.clone()
    } else {
        // Check if parent exists and is within bounds
        path.parent()
            .ok_or_else(|| ToolError::InvalidInput("Invalid path: no parent directory".into()))?
            .to_path_buf()
    };

    // Canonicalize paths to resolve symlinks and ".." components
    let canonical_wd = working_directory.canonicalize().map_err(|e| {
        ToolError::Internal(format!("Failed to canonicalize working directory: {}", e))
    })?;

    let canonical_path = if path_to_check.exists() {
        path_to_check.canonicalize().map_err(|e| {
            ToolError::InvalidInput(format!("Failed to resolve path: {}", e))
        })?
    } else {
        // If parent doesn't exist either, it's invalid
        return Err(ToolError::InvalidInput(format!(
            "Parent directory does not exist: {}",
            path_to_check.display()
        )));
    };

    // Check that the canonical path is within the working directory
    if !canonical_path.starts_with(&canonical_wd) {
        return Err(ToolError::PermissionDenied(format!(
            "Path '{}' is outside the working directory",
            requested_path
        )));
    }

    // Return the original resolved path (not canonicalized) for display purposes
    // but we've validated it's safe
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_error_display() {
        let err = ToolError::NotFound("test_tool".to_string());
        assert_eq!(err.to_string(), "Tool not found: test_tool");

        let err = ToolError::PermissionDenied("dangerous_operation".to_string());
        assert_eq!(err.to_string(), "Permission denied: dangerous_operation");
    }
}
