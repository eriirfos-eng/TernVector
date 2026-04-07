// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Network Configuration and Status

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub endpoints: Vec<String>,
    pub min_peers: usize,
    pub max_peers: usize,
    pub heartbeat_interval_ms: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            endpoints: vec!["https://qudag.network:8443".to_string()],
            min_peers: 3,
            max_peers: 50,
            heartbeat_interval_ms: 30000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkStatus {
    pub connected: bool,
    pub peer_count: usize,
    pub latest_round: u64,
    pub sync_status: SyncStatus,
    pub network_version: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyncStatus {
    Synced,
    Syncing,
    Behind,
    Disconnected,
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncStatus::Synced => write!(f, "synced"),
            SyncStatus::Syncing => write!(f, "syncing"),
            SyncStatus::Behind => write!(f, "behind"),
            SyncStatus::Disconnected => write!(f, "disconnected"),
        }
    }
}
