// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! # REFRAG Pipeline Example
//!
//! This example demonstrates the REFRAG (Rethinking RAG) framework for ~30x latency reduction
//! in Retrieval-Augmented Generation systems.
//!
//! ## Architecture
//!
//! The pipeline consists of three layers:
//!
//! 1. **Compress Layer**: Stores pre-computed "Chunk Embeddings" as binary tensors
//! 2. **Sense Layer**: Policy network decides whether to return tensor or text
//! 3. **Expand Layer**: Projects tensors to target LLM dimensions if needed
//!
//! ## Usage
//!
//! ```rust,ignore
//! use refrag_pipeline_example::{RefragStore, RefragEntry};
//!
//! // Create REFRAG-enabled store
//! let store = RefragStore::new(768, 4096).unwrap();
//!
//! // Insert with representation tensor
//! let entry = RefragEntry::new("doc_1", vec![0.1; 768], "The quick brown fox...")
//!     .with_tensor(vec![0u8; 768 * 4], "llama3-8b");
//! store.insert(entry).unwrap();
//!
//! // Search with policy-based routing
//! let query = vec![0.1; 768];
//! let results = store.search_hybrid(&query, 10, Some(0.85)).unwrap();
//! ```

pub mod compress;
pub mod expand;
pub mod sense;
pub mod store;
pub mod types;

pub use compress::TensorCompressor;
pub use expand::Projector;
pub use sense::{PolicyNetwork, RefragAction};
pub use store::RefragStore;
pub use types::{RefragEntry, RefragResponseType, RefragSearchResult};
