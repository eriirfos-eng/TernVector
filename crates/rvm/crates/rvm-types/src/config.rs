// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Global RVM configuration constants and defaults.

use crate::CoherenceScore;

/// Top-level RVM configuration.
#[derive(Debug, Clone, Copy)]
pub struct RvmConfig {
    /// Maximum partitions (DC-12).
    pub max_partitions: u16,
    /// Default coherence threshold.
    pub coherence_threshold: CoherenceScore,
    /// Witness ring buffer capacity in records.
    pub witness_ring_capacity: usize,
    /// Scheduler epoch interval in nanoseconds.
    pub epoch_interval_ns: u64,
}

impl Default for RvmConfig {
    fn default() -> Self {
        Self {
            max_partitions: 256,
            coherence_threshold: CoherenceScore::DEFAULT_THRESHOLD,
            witness_ring_capacity: 262_144,
            epoch_interval_ns: 10_000_000, // 10 ms
        }
    }
}
