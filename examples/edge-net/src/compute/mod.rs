// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! SIMD Compute Backend for edge-net P2P AI Network
//!
//! Provides portable CPU acceleration with support for:
//! - WASM simd128 intrinsics (browser/WASM targets)
//! - x86_64 AVX2 intrinsics (native x86 targets)
//! - Scalar fallback for unsupported platforms
//!
//! Performance targets:
//! - 2,236+ ops/sec for MicroLoRA (rank-2)
//! - 150x faster HNSW search
//! - Q4 quantized inference

pub mod simd;

pub use simd::*;
