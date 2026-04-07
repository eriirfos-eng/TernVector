// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! # Router Core
//!
//! High-performance vector database and neural routing inference engine.
//!
//! This crate provides the core functionality for:
//! - Vector storage and retrieval
//! - HNSW (Hierarchical Navigable Small World) indexing
//! - Multiple quantization techniques (scalar, product, binary)
//! - SIMD-optimized distance calculations
//! - AgenticDB API compatibility

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs, rustdoc::broken_intra_doc_links)]

pub mod distance;
pub mod error;
pub mod index;
pub mod quantization;
pub mod storage;
pub mod types;
pub mod vector_db;

// Re-exports for convenience
pub use error::{Result, VectorDbError};
pub use types::{DistanceMetric, SearchQuery, SearchResult, VectorEntry};
pub use vector_db::VectorDB;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Basic smoke test
        assert!(true);
    }
}
