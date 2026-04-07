// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Degraded mode when coherence engine is unavailable.

/// Reason for entering degraded mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DegradedReason {
    /// Coherence engine is not available (DC-1/DC-6).
    CoherenceUnavailable,
    /// MinCut budget was exceeded.
    MinCutBudgetExceeded,
    /// Recovery mode triggered.
    RecoveryTriggered,
}

/// State of the degraded scheduler.
#[derive(Debug, Clone, Copy)]
pub struct DegradedState {
    /// Reason for degradation.
    pub reason: DegradedReason,
    /// Epoch at which degradation began.
    pub since_epoch: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degraded_reasons() {
        assert_ne!(DegradedReason::CoherenceUnavailable, DegradedReason::MinCutBudgetExceeded);
        assert_ne!(DegradedReason::MinCutBudgetExceeded, DegradedReason::RecoveryTriggered);
    }

    #[test]
    fn test_degraded_state_creation() {
        let state = DegradedState {
            reason: DegradedReason::CoherenceUnavailable,
            since_epoch: 5,
        };
        assert_eq!(state.reason, DegradedReason::CoherenceUnavailable);
        assert_eq!(state.since_epoch, 5);
    }
}
