// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! WASM bindings and optimizations for agentic chip
//!
//! Provides:
//! - SIMD-accelerated boundary computation
//! - Agentic chip interface
//! - Inter-core messaging
//! - Canonical min-cut FFI (ADR-117)

pub mod agentic;
pub mod simd;

#[cfg(feature = "canonical")]
pub mod canonical;

pub use agentic::*;
pub use simd::*;
