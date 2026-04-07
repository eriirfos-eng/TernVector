// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Communication edges between partitions.

use rvm_types::PartitionId;

/// Unique identifier for a communication edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CommEdgeId(u64);

impl CommEdgeId {
    /// Create a new communication edge identifier.
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Return the raw identifier value.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

/// A weighted communication edge between two partitions.
#[derive(Debug, Clone, Copy)]
pub struct CommEdge {
    /// Unique identifier for this edge.
    pub id: CommEdgeId,
    /// Source partition.
    pub source: PartitionId,
    /// Destination partition.
    pub dest: PartitionId,
    /// Edge weight (accumulated message bytes, decayed per epoch).
    pub weight: u64,
    /// Epoch in which this edge was last updated.
    pub last_epoch: u32,
}
