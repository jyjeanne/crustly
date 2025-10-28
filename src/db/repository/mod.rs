//! Repository Module
//!
//! Repository pattern implementations for database access.

pub mod session;
pub mod message;
pub mod file;

pub use session::{SessionRepository, SessionListOptions};
pub use message::MessageRepository;
pub use file::FileRepository;

use anyhow::Result;

/// Repository trait for common database operations
#[async_trait::async_trait]
pub trait Repository<T> {
    /// Find entity by ID
    async fn find_by_id(&self, id: &str) -> Result<Option<T>>;

    /// Create a new entity
    async fn create(&self, entity: &T) -> Result<()>;

    /// Update an existing entity
    async fn update(&self, entity: &T) -> Result<()>;

    /// Delete an entity by ID
    async fn delete(&self, id: &str) -> Result<()>;

    /// List all entities
    async fn list(&self) -> Result<Vec<T>>;
}
