// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Mixed Curvature Attention
//!
//! Attention in product spaces: E^e × H^h × S^s
//!
//! ## Key Optimizations
//!
//! 1. **Tangent Space Mapping**: Map hyperbolic to tangent space at origin
//! 2. **Fused Dot Kernel**: Single vectorized loop for all three similarities
//! 3. **Per-Head Mixing**: Low-rank learned weights per head
//! 4. **Quantization-Friendly**: Different precision for each component

mod component_quantizer;
mod fused_attention;
mod tangent_space;

pub use component_quantizer::{ComponentQuantizer, QuantizationConfig, QuantizedVector};
pub use fused_attention::{
    FusedCurvatureConfig, MixedCurvatureCache, MixedCurvatureFusedAttention,
};
pub use tangent_space::{TangentSpaceConfig, TangentSpaceMapper};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        assert!(true);
    }
}
