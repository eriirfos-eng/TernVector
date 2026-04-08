// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! PDE-Based Attention
//!
//! Continuous-time attention using partial differential equations.
//!
//! ## Key Concepts
//!
//! 1. **Diffusion Smoothing**: Heat equation on attention graph
//! 2. **Graph Laplacian**: L = D - W for key similarity
//! 3. **Time Evolution**: x_{t+dt} = x_t - dt * L * x_t
//!
//! ## Interpretation
//!
//! - Attention as continuous information flow
//! - Smoothing removes noise while preserving structure
//! - Multi-scale attention via different diffusion times

mod diffusion;
mod laplacian;

pub use diffusion::{DiffusionAttention, DiffusionConfig};
pub use laplacian::{GraphLaplacian, LaplacianType};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        assert!(true);
    }
}
