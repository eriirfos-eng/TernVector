// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Pipeline API for chaining attention operations.

use crate::{error::AttentionResult, traits::Attention};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NormType {
    LayerNorm,
    RMSNorm,
    BatchNorm,
}

pub enum PipelineStage {
    Attention(Box<dyn Attention + Send + Sync>),
    Normalize(NormType),
}

pub struct AttentionPipeline {
    stages: Vec<PipelineStage>,
}

impl AttentionPipeline {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    pub fn add_attention(mut self, attn: Box<dyn Attention + Send + Sync>) -> Self {
        self.stages.push(PipelineStage::Attention(attn));
        self
    }

    pub fn add_norm(mut self, norm: NormType) -> Self {
        self.stages.push(PipelineStage::Normalize(norm));
        self
    }

    pub fn add_dropout(self, _p: f32) -> Self {
        self
    }
    pub fn add_residual(self) -> Self {
        self
    }

    pub fn run(
        &self,
        query: &[f32],
        _keys: &[&[f32]],
        _values: &[&[f32]],
    ) -> AttentionResult<Vec<f32>> {
        Ok(query.to_vec())
    }
}

impl Default for AttentionPipeline {
    fn default() -> Self {
        Self::new()
    }
}
