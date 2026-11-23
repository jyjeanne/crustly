//! Long-term memory: durable facts and preferences kept in the Crustly
//! database and recalled into later requests (docs/improve-memory.md).

pub mod model;

pub use model::{
    content_hash, normalize, validate, Memory, MemoryCategory, MemoryScope, MemorySource,
    MemoryStatus, NewMemory, ProjectId, RejectReason, SaveOutcome,
};
