// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

// Tiny Dancer Routing Module
//
// Neural-powered dynamic agent routing with FastGRNN for adaptive decision-making.

pub mod agents;
pub mod fastgrnn;
pub mod operators;
pub mod router;

pub use agents::{Agent, AgentRegistry, AgentType};
pub use fastgrnn::FastGRNN;
pub use router::{OptimizationTarget, Router, RoutingConstraints, RoutingDecision};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify all types are exported
        let _registry = AgentRegistry::new();
        let _router = Router::new();
    }
}
