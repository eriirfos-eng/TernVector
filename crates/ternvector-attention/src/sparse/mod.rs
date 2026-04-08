// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Sparse attention mechanisms for efficient computation on long sequences
//!
//! This module provides sparse attention patterns that reduce complexity from O(n²) to sub-quadratic.

pub mod flash;
pub mod linear;
pub mod local_global;
pub mod mask;

pub use flash::FlashAttention;
pub use linear::LinearAttention;
pub use local_global::LocalGlobalAttention;
pub use mask::{AttentionMask, SparseMaskBuilder};
