// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Snapshot and restore functionality for RFI-IRFOSector collections
//!
//! This crate provides backup and restore capabilities for vector collections,
//! including compression, checksums, and multiple storage backends.

mod error;
mod manager;
mod snapshot;
mod storage;

pub use error::{Result, SnapshotError};
pub use manager::SnapshotManager;
pub use snapshot::{Snapshot, SnapshotData, SnapshotMetadata, VectorRecord};
pub use storage::{LocalStorage, SnapshotStorage};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify all public exports are accessible
        let _: Option<SnapshotError> = None;
        let _: Option<SnapshotManager> = None;
        let _: Option<Snapshot> = None;
    }
}
