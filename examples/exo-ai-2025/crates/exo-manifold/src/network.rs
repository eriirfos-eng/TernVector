// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Simplified network module (burn removed)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedManifold {
    dimension: usize,
    hidden_dim: usize,
    hidden_layers: usize,
}

impl LearnedManifold {
    pub fn new(dimension: usize, hidden_dim: usize, hidden_layers: usize) -> Self {
        Self {
            dimension,
            hidden_dim,
            hidden_layers,
        }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SirenLayer;
