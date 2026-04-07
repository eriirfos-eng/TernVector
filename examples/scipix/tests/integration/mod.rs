// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

// Integration test module organization
//
// This module provides integration tests for the ruvector-scipix OCR system.
// Tests are organized by functionality area.

pub mod accuracy_tests;
pub mod api_tests;
pub mod cache_tests;
pub mod cli_tests;
pub mod performance_tests;
pub mod pipeline_tests;

// Re-export common test utilities
pub use crate::common::*;
