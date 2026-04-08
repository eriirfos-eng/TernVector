// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Topology Gated Attention
//!
//! Uses topological structure as a permission signal for attention.
//!
//! ## Key Concepts
//!
//! 1. **Window-Level Coherence**: Compute one coherence score per window, reuse for all queries
//! 2. **Fast Graph Primitives**: Use k-NN lists instead of full graph construction
//! 3. **3-Mode Policy**: stable/cautious/freeze based on coherence
//! 4. **Amortized Updates**: Update coherence every T tokens, not every token
//!
//! ## Modes
//!
//! - **Stable**: Full attention, normal updates
//! - **Cautious**: Reduced attention width, increased sparsity
//! - **Freeze**: Retrieval only, no updates, no writes

mod coherence;
mod gated_attention;
mod policy;

pub use coherence::{CoherenceMetric, WindowCoherence};
pub use gated_attention::{TopologyGatedAttention, TopologyGatedConfig};
pub use policy::{AttentionMode, AttentionPolicy, PolicyConfig};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        assert!(true);
    }
}
