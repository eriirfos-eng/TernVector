// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Vector storage, embedding engine, and trait abstractions.
//!
//! Provides HNSW-backed vector storage for captured frames with
//! cosine similarity search, metadata filtering, delete/update operations,
//! and a pluggable embedding model trait.

pub mod embedding;
pub mod traits;
pub mod vector_store;

pub use embedding::EmbeddingEngine;
pub use traits::{EmbeddingModel, HashEmbeddingModel};
pub use vector_store::{SearchFilter, SearchResult, StoredEmbedding, VectorStore};

#[cfg(not(target_arch = "wasm32"))]
pub use traits::RuvectorEmbeddingModel;
#[cfg(not(target_arch = "wasm32"))]
pub use vector_store::HnswVectorStore;
