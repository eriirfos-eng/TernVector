// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Modern Hopfield Networks
//!
//! This module implements the modern Hopfield network formulation from
//! Ramsauer et al. (2020), which provides exponential storage capacity
//! and is mathematically equivalent to transformer attention.
//!
//! ## Components
//!
//! - [`ModernHopfield`]: Main network structure
//! - [`retrieval`]: Softmax-weighted retrieval implementation
//! - [`capacity`]: Capacity calculations and β tuning

mod capacity;
mod network;
mod retrieval;

pub use capacity::{optimal_beta, theoretical_capacity};
pub use network::ModernHopfield;
pub use retrieval::{compute_attention, softmax};

#[cfg(test)]
mod tests;
