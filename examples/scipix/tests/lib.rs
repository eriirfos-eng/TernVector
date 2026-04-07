// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

// Integration test library for ruvector-scipix
//
// This library provides the test infrastructure and utilities
// for integration testing the scipix OCR system.

// Common test utilities
pub mod common;

// Integration test modules
pub mod integration;

// Test configuration
#[cfg(test)]
mod test_config {
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Initialize test environment once
    pub fn init() {
        INIT.call_once(|| {
            // Setup test logging
            let _ = env_logger::builder().is_test(true).try_init();

            // Create test directories
            let test_dirs = vec![
                "/tmp/scipix_test",
                "/tmp/scipix_cache",
                "/tmp/scipix_results",
            ];

            for dir in test_dirs {
                std::fs::create_dir_all(dir).ok();
            }
        });
    }
}

// Convenience re-exports for tests
pub use common::*;
