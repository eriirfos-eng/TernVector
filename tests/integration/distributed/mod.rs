// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Distributed Systems Integration Tests
//!
//! Comprehensive test suite for horizontal scaling components:
//! - Raft consensus protocol
//! - Multi-master replication
//! - Auto-sharding with consistent hashing
//!
//! These tests simulate a distributed environment similar to E2B sandboxes

pub mod raft_consensus_tests;
pub mod replication_tests;
pub mod sharding_tests;
pub mod cluster_integration_tests;
pub mod performance_benchmarks;
