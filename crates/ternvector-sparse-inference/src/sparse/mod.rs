// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Sparse computation module.
//!
//! This module provides sparse implementations of neural network layers.

mod ffn;

pub use crate::config::ActivationType;
pub use ffn::SparseFfn;

/// Trait for feed-forward network layers.
pub trait FeedForward: Send + Sync {
    /// Sparse forward pass using only active neurons.
    fn forward_sparse(
        &self,
        input: &[f32],
        active_neurons: &[usize],
    ) -> crate::error::Result<Vec<f32>>;

    /// Dense forward pass using all neurons.
    fn forward_dense(&self, input: &[f32]) -> crate::error::Result<Vec<f32>>;
}

impl FeedForward for SparseFfn {
    fn forward_sparse(
        &self,
        input: &[f32],
        active_neurons: &[usize],
    ) -> crate::error::Result<Vec<f32>> {
        SparseFfn::forward_sparse(self, input, active_neurons)
    }

    fn forward_dense(&self, input: &[f32]) -> crate::error::Result<Vec<f32>> {
        SparseFfn::forward_dense(self, input)
    }
}

/// SwiGLU FFN (placeholder for future implementation).
pub struct SwiGLUFfn;

impl SwiGLUFfn {
    /// Create a new SwiGLU FFN.
    pub fn new(_input_dim: usize, _hidden_dim: usize) -> Self {
        Self
    }
}

impl FeedForward for SwiGLUFfn {
    fn forward_sparse(
        &self,
        _input: &[f32],
        _active_neurons: &[usize],
    ) -> crate::error::Result<Vec<f32>> {
        unimplemented!("SwiGLUFfn not yet implemented")
    }

    fn forward_dense(&self, _input: &[f32]) -> crate::error::Result<Vec<f32>> {
        unimplemented!("SwiGLUFfn not yet implemented")
    }
}
