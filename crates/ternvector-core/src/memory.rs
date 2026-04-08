// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Memory management utilities for ruvector-core
//!
//! This module provides memory-efficient data structures and utilities
//! for vector storage operations.

/// Memory pool for vector allocations.
#[derive(Debug, Default)]
pub struct MemoryPool {
    /// Total allocated bytes.
    allocated: usize,
    /// Maximum allocation limit.
    limit: Option<usize>,
}

impl MemoryPool {
    /// Create a new memory pool.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a memory pool with a limit.
    pub fn with_limit(limit: usize) -> Self {
        Self {
            allocated: 0,
            limit: Some(limit),
        }
    }

    /// Get currently allocated bytes.
    pub fn allocated(&self) -> usize {
        self.allocated
    }

    /// Get the allocation limit, if any.
    pub fn limit(&self) -> Option<usize> {
        self.limit
    }
}
