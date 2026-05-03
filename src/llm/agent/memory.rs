//! Agent memory: episodic cross-session summaries and codebase symbol index.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;
use uuid::Uuid;

// ── Episodic Memory ──────────────────────────────────────────────────────────

/// A compressed summary of one past session, injected at session start.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub id: Uuid,
    pub session_id: Uuid,
    pub summary_text: String,
    pub token_count: i32,
    pub files_touched: Vec<String>,
    pub decisions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

// ── Codebase Index ───────────────────────────────────────────────────────────

/// Kind of a symbol extracted by tree-sitter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolKind {
    Fn,
    Struct,
    Enum,
    Trait,
    Mod,
    Impl,
    Const,
}

/// One entry in the codebase symbol index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebaseIndexEntry {
    pub id: Uuid,
    pub file_path: String,
    pub symbol_name: String,
    pub symbol_kind: SymbolKind,
    pub line_number: i32,
    pub indexed_at: DateTime<Utc>,
}

/// Persistent, incrementally-updated index of project symbols.
pub struct CodebaseIndex {
    pool: SqlitePool,
}

impl CodebaseIndex {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Index all symbols in a single file, replacing any prior entries for that path.
    pub async fn index_file(&self, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy().to_string();
        let content = tokio::fs::read_to_string(path).await?;
        let entries = extract_symbols(&path_str, &content);

        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM codebase_index WHERE file_path = ?")
            .bind(&path_str)
            .execute(&mut *tx)
            .await?;

        for entry in &entries {
            sqlx::query(
                "INSERT OR REPLACE INTO codebase_index \
                 (id, file_path, symbol_name, symbol_kind, line_number, indexed_at) \
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(entry.id.to_string())
            .bind(&entry.file_path)
            .bind(&entry.symbol_name)
            .bind(symbol_kind_str(&entry.symbol_kind))
            .bind(entry.line_number)
            .bind(entry.indexed_at.timestamp())
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Look up entries by exact symbol name.
    pub async fn query_symbol(&self, name: &str) -> Result<Vec<CodebaseIndexEntry>> {
        let rows: Vec<(String, String, String, String, i64, i64)> = sqlx::query_as(
            "SELECT id, file_path, symbol_name, symbol_kind, line_number, indexed_at \
             FROM codebase_index WHERE symbol_name = ?",
        )
        .bind(name)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(row_to_entry).collect())
    }

    /// Full-text search across symbol names and file paths.
    pub async fn fts_search(&self, query: &str) -> Result<Vec<CodebaseIndexEntry>> {
        let pattern = format!("%{}%", query.replace(' ', "%"));
        let rows: Vec<(String, String, String, String, i64, i64)> = sqlx::query_as(
            "SELECT id, file_path, symbol_name, symbol_kind, line_number, indexed_at \
             FROM codebase_index \
             WHERE symbol_name LIKE ? OR file_path LIKE ? \
             ORDER BY symbol_name LIMIT 50",
        )
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(row_to_entry).collect())
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn symbol_kind_str(kind: &SymbolKind) -> &'static str {
    match kind {
        SymbolKind::Fn => "fn",
        SymbolKind::Struct => "struct",
        SymbolKind::Enum => "enum",
        SymbolKind::Trait => "trait",
        SymbolKind::Mod => "mod",
        SymbolKind::Impl => "impl",
        SymbolKind::Const => "const",
    }
}

fn str_to_symbol_kind(s: &str) -> SymbolKind {
    match s {
        "fn" => SymbolKind::Fn,
        "struct" => SymbolKind::Struct,
        "enum" => SymbolKind::Enum,
        "trait" => SymbolKind::Trait,
        "mod" => SymbolKind::Mod,
        "impl" => SymbolKind::Impl,
        _ => SymbolKind::Const,
    }
}

fn row_to_entry(row: (String, String, String, String, i64, i64)) -> CodebaseIndexEntry {
    let (id, file_path, symbol_name, symbol_kind, line_number, indexed_at) = row;
    CodebaseIndexEntry {
        id: id.parse().unwrap_or_else(|_| Uuid::new_v4()),
        file_path,
        symbol_name,
        symbol_kind: str_to_symbol_kind(&symbol_kind),
        line_number: line_number as i32,
        indexed_at: DateTime::from_timestamp(indexed_at, 0).unwrap_or_else(Utc::now),
    }
}

/// Lightweight regex-based symbol extractor for Rust files.
fn extract_symbols(file_path: &str, content: &str) -> Vec<CodebaseIndexEntry> {
    let patterns: &[(&str, SymbolKind)] = &[
        (r"^pub\s+fn\s+(\w+)", SymbolKind::Fn),
        (r"^\s+fn\s+(\w+)", SymbolKind::Fn),
        (r"^pub\s+struct\s+(\w+)", SymbolKind::Struct),
        (r"^pub\s+enum\s+(\w+)", SymbolKind::Enum),
        (r"^pub\s+trait\s+(\w+)", SymbolKind::Trait),
        (r"^pub\s+mod\s+(\w+)", SymbolKind::Mod),
        (r"^impl\s+(\w+)", SymbolKind::Impl),
        (r"^pub\s+const\s+(\w+)", SymbolKind::Const),
    ];

    let mut entries = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        for (pattern, kind) in patterns {
            if let Some(caps) = regex::Regex::new(pattern).ok().and_then(|re| re.captures(line)) {
                if let Some(name) = caps.get(1) {
                    entries.push(CodebaseIndexEntry {
                        id: Uuid::new_v4(),
                        file_path: file_path.to_string(),
                        symbol_name: name.as_str().to_string(),
                        symbol_kind: kind.clone(),
                        line_number: (line_idx + 1) as i32,
                        indexed_at: Utc::now(),
                    });
                }
            }
        }
    }

    entries
}
