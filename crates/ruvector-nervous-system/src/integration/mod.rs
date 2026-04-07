// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Integration layer connecting nervous system components to TernVector
//!
//! This module provides the integration layer that maps nervous system concepts
//! to TernVector operations:
//!
//! - **Hopfield retrieval** → Additional index lane alongside HNSW
//! - **Pattern separation** → Sparse encoding before indexing
//! - **BTSP** → One-shot vector index updates
//! - **Predictive residual** → Writes only when prediction violated
//! - **Collection versioning** → Parameter versioning with EWC
//!
//! # Example
//!
//! ```rust,ignore
//! use ruvector_nervous_system::integration::{NervousVectorIndex, NervousConfig};
//!
//! // Create hybrid index with nervous system features
//! let config = NervousConfig::default();
//! let mut index = NervousVectorIndex::new(128, config);
//!
//! // Insert with pattern separation
//! let vector = vec![0.5; 128];
//! index.insert(&vector, Some("metadata"));
//!
//! // Hybrid search (Hopfield + HNSW)
//! let results = index.search_hybrid(&vector, 10);
//!
//! // One-shot learning
//! let key = vec![0.1; 128];
//! let value = vec![0.9; 128];
//! index.learn_one_shot(&key, &value);
//! ```

pub mod postgres;
pub mod ruvector;
pub mod versioning;

pub use postgres::{PredictiveConfig, PredictiveWriter};
pub use ruvector::{HybridSearchResult, NervousConfig, NervousVectorIndex};
pub use versioning::{
    CollectionVersioning, ConsolidationSchedule, EligibilityState, ParameterVersion,
};

#[cfg(test)]
mod tests;
