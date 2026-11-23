//! Vector helpers for semantic recall (docs/improve-memory.md §6.2, §11.4,
//! tasks P2.5 and P2.8).
//!
//! Vectors are L2-normalised before they are stored, so cosine similarity is
//! a plain dot product. They are stored as little-endian `f32` bytes, with
//! the dimension count kept beside them, so a stored vector is checked before
//! it is compared with anything.

use std::fmt;

/// A problem with a vector: wrong size, wrong byte length, or zero length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorError {
    Empty,
    /// The bytes do not hold the stated number of `f32` values.
    BadLength {
        expected: usize,
        got: usize,
    },
    /// Two vectors of different sizes cannot be compared.
    DimensionMismatch {
        left: usize,
        right: usize,
    },
}

impl fmt::Display for VectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "the vector is empty"),
            Self::BadLength { expected, got } => {
                write!(f, "expected {expected} bytes for the vector, got {got}")
            }
            Self::DimensionMismatch { left, right } => {
                write!(
                    f,
                    "vectors of {left} and {right} dimensions cannot be compared"
                )
            }
        }
    }
}

impl std::error::Error for VectorError {}

/// Scale `v` to unit length. A zero vector has no direction, so it is refused.
pub fn normalize(v: &[f32]) -> Result<Vec<f32>, VectorError> {
    if v.is_empty() {
        return Err(VectorError::Empty);
    }
    let norm = v
        .iter()
        .map(|x| f64::from(*x) * f64::from(*x))
        .sum::<f64>()
        .sqrt();
    if norm == 0.0 || !norm.is_finite() {
        return Err(VectorError::Empty);
    }
    Ok(v.iter().map(|x| (f64::from(*x) / norm) as f32).collect())
}

/// Little-endian bytes of `v`, four per value.
pub fn encode(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

/// Read `dimensions` little-endian values from `bytes`. Any other length is
/// an error, never a silently truncated or padded vector.
pub fn decode(bytes: &[u8], dimensions: usize) -> Result<Vec<f32>, VectorError> {
    let expected = dimensions * 4;
    if bytes.len() != expected {
        return Err(VectorError::BadLength {
            expected,
            got: bytes.len(),
        });
    }
    Ok(bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect())
}

/// Cosine similarity of two vectors that are already unit length: their dot
/// product. Refuses vectors of different sizes.
pub fn similarity(a: &[f32], b: &[f32]) -> Result<f32, VectorError> {
    if a.len() != b.len() {
        return Err(VectorError::DimensionMismatch {
            left: a.len(),
            right: b.len(),
        });
    }
    Ok(a.iter().zip(b).map(|(x, y)| x * y).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_vectors_have_unit_length() {
        let v = normalize(&[3.0, 4.0]).unwrap();
        let length = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((length - 1.0).abs() < 1e-6, "{length}");
        assert!((v[0] - 0.6).abs() < 1e-6 && (v[1] - 0.8).abs() < 1e-6);
    }

    #[test]
    fn zero_and_empty_vectors_are_refused() {
        assert_eq!(normalize(&[]), Err(VectorError::Empty));
        assert_eq!(normalize(&[0.0, 0.0]), Err(VectorError::Empty));
    }

    #[test]
    fn encode_then_decode_round_trips_exactly() {
        let v = vec![0.25_f32, -1.5, 3.0e-3, 42.0];
        let bytes = encode(&v);
        assert_eq!(bytes.len(), 16);
        assert_eq!(decode(&bytes, 4).unwrap(), v);
    }

    #[test]
    fn encoding_is_little_endian() {
        assert_eq!(encode(&[1.0]), 1.0_f32.to_le_bytes().to_vec());
    }

    #[test]
    fn decode_refuses_a_length_that_does_not_match() {
        assert_eq!(
            decode(&[0u8; 7], 2),
            Err(VectorError::BadLength {
                expected: 8,
                got: 7
            })
        );
        assert!(decode(&[0u8; 8], 3).is_err());
    }

    #[test]
    fn similarity_is_the_dot_product_of_unit_vectors() {
        let a = normalize(&[1.0, 0.0]).unwrap();
        let b = normalize(&[1.0, 1.0]).unwrap();
        let s = similarity(&a, &b).unwrap();
        assert!((s - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-6, "{s}");
        assert!((similarity(&a, &a).unwrap() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn similarity_refuses_different_dimensions() {
        assert_eq!(
            similarity(&[1.0], &[1.0, 0.0]),
            Err(VectorError::DimensionMismatch { left: 1, right: 2 })
        );
    }
}
