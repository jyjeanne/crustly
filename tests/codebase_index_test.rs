//! Integration tests for CodebaseIndex (QS-4.3, FR-011).
//!
//! Run with: cargo test codebase_index

use crustly::llm::agent::memory::{CodebaseIndex, SymbolKind};
use std::path::Path;

async fn create_pool_with_schema() -> sqlx::SqlitePool {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query(
        "CREATE TABLE codebase_index (
            id TEXT PRIMARY KEY,
            file_path TEXT NOT NULL,
            symbol_name TEXT NOT NULL,
            symbol_kind TEXT NOT NULL,
            line_number INTEGER NOT NULL,
            indexed_at INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
    pool
}

/// QS-4.3: Index a Rust file, query by symbol name, verify result.
///
/// `trait.rs` defines `pub trait Provider` — the LLM provider interface.
#[tokio::test]
async fn index_and_query_provider_trait() {
    let pool = create_pool_with_schema().await;
    let index = CodebaseIndex::new(pool);

    let trait_rs = Path::new("src/llm/provider/trait.rs");
    index
        .index_file(trait_rs)
        .await
        .expect("index_file must succeed");

    // The trait is named "Provider" in trait.rs
    let results = index
        .query_symbol("Provider")
        .await
        .expect("query must succeed");

    assert!(
        !results.is_empty(),
        "Provider trait must be found in the index after indexing trait.rs"
    );

    let entry = &results[0];
    assert_eq!(entry.symbol_name, "Provider");
    assert_eq!(entry.symbol_kind, SymbolKind::Trait);
    assert!(entry.line_number > 0, "line number must be set");
    assert!(entry.file_path.contains("trait.rs"));
}

/// Indexing a file twice must not duplicate entries.
#[tokio::test]
async fn index_file_twice_no_duplicate() {
    let pool = create_pool_with_schema().await;
    let index = CodebaseIndex::new(pool.clone());

    let trait_rs = Path::new("src/llm/provider/trait.rs");
    index.index_file(trait_rs).await.expect("first index");
    index.index_file(trait_rs).await.expect("second index");

    let results = index.query_symbol("Provider").await.expect("query");
    let count = results
        .iter()
        .filter(|e| e.symbol_name == "Provider")
        .count();
    assert_eq!(
        count, 1,
        "Provider must appear exactly once after two index passes"
    );
}

/// FTS search must find symbols by partial name.
#[tokio::test]
async fn fts_search_finds_symbol_by_partial_name() {
    let pool = create_pool_with_schema().await;
    let index = CodebaseIndex::new(pool);

    let trait_rs = Path::new("src/llm/provider/trait.rs");
    index.index_file(trait_rs).await.expect("index");

    // "Provider" is the trait name in trait.rs; fts_search matches on symbol_name LIKE '%Provider%'
    let results = index.fts_search("Provider").await.expect("fts_search");
    assert!(
        !results.is_empty(),
        "fts_search('Provider') must return at least one result"
    );
    let found = results
        .iter()
        .any(|e| e.symbol_name.contains("Provider") || e.file_path.contains("trait.rs"));
    assert!(
        found,
        "at least one result must match 'Provider' symbol or trait.rs path"
    );
}

/// Indexing a non-existent file must return an error.
#[tokio::test]
async fn index_nonexistent_file_returns_error() {
    let pool = create_pool_with_schema().await;
    let index = CodebaseIndex::new(pool);

    let result = index.index_file(Path::new("nonexistent_file_xyz.rs")).await;
    assert!(
        result.is_err(),
        "index_file must fail for a non-existent file"
    );
}
