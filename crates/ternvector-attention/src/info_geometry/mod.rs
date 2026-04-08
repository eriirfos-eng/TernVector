// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Information Geometry for Attention
//!
//! Natural gradient methods using Fisher information metric.
//!
//! ## Key Concepts
//!
//! 1. **Fisher Metric**: F = diag(p) - p*p^T on probability simplex
//! 2. **Natural Gradient**: Solve F*delta = grad, then update params -= lr*delta
//! 3. **Conjugate Gradient**: Efficient solver for Fisher system
//!
//! ## Use Cases
//!
//! - Training attention weights with proper geometry
//! - Routing probabilities in MoE
//! - Softmax logit optimization

mod fisher;
mod natural_gradient;

pub use fisher::{FisherConfig, FisherMetric};
pub use natural_gradient::{NaturalGradient, NaturalGradientConfig};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        assert!(true);
    }
}
