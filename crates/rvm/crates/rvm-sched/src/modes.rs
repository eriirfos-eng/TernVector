// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Scheduler operating modes.

/// Scheduler operating mode (ADR-132).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerMode {
    /// Hard real-time. Bounded local execution only.
    Reflex,
    /// Normal execution with coherence-aware placement.
    Flow,
    /// Stabilization: replay, rollback, split.
    Recovery,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_equality() {
        assert_eq!(SchedulerMode::Reflex, SchedulerMode::Reflex);
        assert_ne!(SchedulerMode::Reflex, SchedulerMode::Flow);
    }

    #[test]
    fn test_mode_variants() {
        let modes = [SchedulerMode::Reflex, SchedulerMode::Flow, SchedulerMode::Recovery];
        assert_eq!(modes.len(), 3);
    }
}
