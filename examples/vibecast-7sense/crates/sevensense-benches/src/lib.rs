// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! 7sense Benchmarks - Performance testing for all bounded contexts
//!
//! This crate contains comprehensive benchmarks for:
//! - HNSW vector search (150x speedup target)
//! - Perch 2.0 embedding inference
//! - Clustering algorithms (HDBSCAN, K-Means)
//! - API endpoint throughput

pub mod utils;

pub use utils::*;
