// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Graph attention mechanisms for GNN applications
//!
//! This module provides graph-specific attention implementations:
//! - Edge-featured attention (GAT with edge features)
//! - Rotary position embeddings for graphs (RoPE)
//! - Dual-space attention (Euclidean + Hyperbolic)

pub mod dual_space;
pub mod edge_featured;
pub mod rope;

pub use dual_space::{DualSpaceAttention, DualSpaceConfig};
pub use edge_featured::{EdgeFeaturedAttention, EdgeFeaturedConfig};
pub use rope::{GraphRoPE, RoPEConfig};
