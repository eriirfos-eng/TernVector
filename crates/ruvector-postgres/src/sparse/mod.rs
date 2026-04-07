// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Sparse vector support for efficient storage and search of high-dimensional sparse embeddings.
//!
//! This module provides:
//! - Sparse vector type with COO (Coordinate) format storage
//! - Efficient sparse-sparse distance computations
//! - PostgreSQL operators and functions
//! - Support for BM25, SPLADE, and learned sparse representations

pub mod distance;
pub mod operators;
pub mod types;

// Re-exports for convenience
pub use distance::{sparse_cosine, sparse_dot, sparse_euclidean};
pub use types::SparseVec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_module() {
        let indices = vec![0, 2, 5];
        let values = vec![1.0, 2.0, 3.0];
        let sparse = SparseVec::new(indices, values, 10).unwrap();

        assert_eq!(sparse.nnz(), 3);
        assert_eq!(sparse.dim(), 10);
    }
}
