//! In-memory brute-force index over memory vectors (docs/improve-memory.md
//! D7, §11.4; task P2.8).
//!
//! One index holds the vectors of one embedding model. Searching it scans
//! every vector: at 1,000 memories of 768 dimensions that is about 3 MB and
//! well under a millisecond, so there is no approximate index to keep in sync.

use super::vector::{similarity, VectorError};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VectorIndex {
    model: String,
    dimensions: usize,
    vectors: HashMap<Uuid, Vec<f32>>,
}

impl VectorIndex {
    /// An empty index for `model`, whose vectors have `dimensions` values.
    pub fn new(model: &str, dimensions: usize) -> Self {
        Self {
            model: model.to_string(),
            dimensions,
            vectors: HashMap::new(),
        }
    }

    /// Build an index from stored vectors. Every vector must have the
    /// index's size: one wrong size would make the index unsafe to search.
    pub fn from_entries(
        model: &str,
        dimensions: usize,
        entries: Vec<(Uuid, Vec<f32>)>,
    ) -> Result<Self, VectorError> {
        let mut index = Self::new(model, dimensions);
        for (id, vector) in entries {
            index.insert(id, vector)?;
        }
        Ok(index)
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    pub fn len(&self) -> usize {
        self.vectors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vectors.is_empty()
    }

    pub fn contains(&self, id: &Uuid) -> bool {
        self.vectors.contains_key(id)
    }

    /// Add or replace a vector. A vector of the wrong size is refused.
    pub fn insert(&mut self, id: Uuid, vector: Vec<f32>) -> Result<(), VectorError> {
        if vector.len() != self.dimensions {
            return Err(VectorError::DimensionMismatch {
                left: self.dimensions,
                right: vector.len(),
            });
        }
        self.vectors.insert(id, vector);
        Ok(())
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.vectors.remove(id);
    }

    /// Similarity between two indexed memories, if both are present.
    pub fn similarity_between(&self, a: &Uuid, b: &Uuid) -> Option<f32> {
        let (va, vb) = (self.vectors.get(a)?, self.vectors.get(b)?);
        similarity(va, vb).ok()
    }

    /// The `k` memories most similar to `query`, best first, keeping only
    /// those at or above `min_similarity`. A query of the wrong size finds
    /// nothing rather than comparing unlike vectors.
    pub fn search(&self, query: &[f32], k: usize, min_similarity: f32) -> Vec<(Uuid, f32)> {
        if query.len() != self.dimensions {
            return Vec::new();
        }
        let mut scored: Vec<(Uuid, f32)> = self
            .vectors
            .iter()
            .filter_map(|(id, v)| similarity(query, v).ok().map(|s| (*id, s)))
            .filter(|(_, s)| *s >= min_similarity)
            .collect();
        scored.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        scored.truncate(k);
        scored
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::vector::normalize;

    fn unit(v: &[f32]) -> Vec<f32> {
        normalize(v).unwrap()
    }

    #[test]
    fn search_returns_the_most_similar_first_and_respects_k() {
        let (a, b, c) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let index = VectorIndex::from_entries(
            "m",
            2,
            vec![
                (a, unit(&[1.0, 0.0])),
                (b, unit(&[1.0, 0.1])),
                (c, unit(&[0.0, 1.0])),
            ],
        )
        .unwrap();
        let hits = index.search(&unit(&[1.0, 0.0]), 2, -1.0);
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].0, a);
        assert_eq!(hits[1].0, b);
    }

    #[test]
    fn search_drops_hits_below_the_threshold() {
        let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
        let index =
            VectorIndex::from_entries("m", 2, vec![(a, unit(&[1.0, 0.0])), (b, unit(&[0.0, 1.0]))])
                .unwrap();
        let hits = index.search(&unit(&[1.0, 0.0]), 10, 0.35);
        assert_eq!(hits.iter().map(|h| h.0).collect::<Vec<_>>(), vec![a]);
    }

    #[test]
    fn wrong_sizes_are_refused_on_insert_and_on_search() {
        let mut index = VectorIndex::new("m", 2);
        assert!(index.insert(Uuid::new_v4(), vec![1.0, 0.0, 0.0]).is_err());
        index.insert(Uuid::new_v4(), unit(&[1.0, 0.0])).unwrap();
        assert!(index.search(&[1.0, 0.0, 0.0], 5, -1.0).is_empty());
    }

    #[test]
    fn insert_replaces_and_remove_deletes() {
        let id = Uuid::new_v4();
        let mut index = VectorIndex::new("m", 2);
        index.insert(id, unit(&[1.0, 0.0])).unwrap();
        index.insert(id, unit(&[0.0, 1.0])).unwrap();
        assert_eq!(index.len(), 1);
        assert!((index.search(&unit(&[0.0, 1.0]), 1, 0.9)[0].1 - 1.0).abs() < 1e-6);
        index.remove(&id);
        assert!(index.is_empty());
    }

    #[test]
    fn similarity_between_two_indexed_memories() {
        let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
        let index =
            VectorIndex::from_entries("m", 2, vec![(a, unit(&[1.0, 0.0])), (b, unit(&[1.0, 0.0]))])
                .unwrap();
        assert!((index.similarity_between(&a, &b).unwrap() - 1.0).abs() < 1e-6);
        assert!(index.similarity_between(&a, &Uuid::new_v4()).is_none());
    }
}
