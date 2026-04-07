// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Progressive HNSW indexing for the TernVector Format (RVF).
//!
//! This crate implements the three-layer progressive indexing model:
//!
//! - **Layer A**: Entry points + coarse routing (< 5ms load, ~0.70 recall)
//! - **Layer B**: Partial adjacency for hot region (100ms-1s load, ~0.85 recall)
//! - **Layer C**: Full HNSW adjacency (seconds load, >= 0.95 recall)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod codec;
pub mod distance;
pub mod hnsw;
pub mod layers;
pub mod progressive;
pub mod traits;

pub use builder::{build_full_index, build_layer_a, build_layer_b, build_layer_c};
pub use codec::{decode_index_seg, encode_index_seg, CodecError, IndexSegData, IndexSegHeader};
pub use distance::{cosine_distance, dot_product, l2_distance};
pub use hnsw::{HnswConfig, HnswGraph, HnswLayer};
pub use layers::{IndexLayer, IndexState, LayerA, LayerB, LayerC, PartitionEntry};
pub use progressive::ProgressiveIndex;
pub use traits::VectorStore;

#[cfg(feature = "std")]
pub use traits::InMemoryVectorStore;
