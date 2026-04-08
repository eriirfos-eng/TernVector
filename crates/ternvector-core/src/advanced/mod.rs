// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! # Advanced Techniques
//!
//! This module contains experimental and advanced features for next-generation vector search:
//! - **Hypergraphs**: n-ary relationships beyond pairwise similarity
//! - **Learned Indexes**: Neural network-based index structures
//! - **Neural Hashing**: Similarity-preserving binary projections
//! - **Topological Data Analysis**: Embedding quality assessment

pub mod hypergraph;
pub mod learned_index;
pub mod neural_hash;
pub mod tda;

pub use hypergraph::{CausalMemory, Hyperedge, HypergraphIndex, TemporalHyperedge};
pub use learned_index::{HybridIndex, LearnedIndex, RecursiveModelIndex};
pub use neural_hash::{DeepHashEmbedding, NeuralHash};
pub use tda::{EmbeddingQuality, TopologicalAnalyzer};
