// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! AgentDB adapter for the TernVector Format (RVF).
//!
//! Maps agentdb's vector storage, HNSW index, and memory pattern APIs
//! onto the RVF segment model:
//!
//! - **VEC_SEG**: Raw vector data (episodes, state embeddings)
//! - **INDEX_SEG**: HNSW index layers (A/B/C progressive indexing)
//! - **META_SEG**: Memory pattern metadata (rewards, critiques, tags)
//!
//! Uses the RVText domain profile for text/embedding workloads.

pub mod index_adapter;
pub mod pattern_store;
pub mod vector_store;

pub use index_adapter::RvfIndexAdapter;
pub use pattern_store::{MemoryPattern, RvfPatternStore};
pub use vector_store::RvfVectorStore;
