// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Common test utilities and helpers for integration tests
//!
//! This module provides shared functionality across all integration tests.

pub mod fixtures;
pub mod assertions;
pub mod helpers;

// Re-export commonly used items
pub use fixtures::*;
pub use assertions::*;
pub use helpers::*;
