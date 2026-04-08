// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! WASM bindings for temporal tensor compression.
//!
//! Re-exports the FFI interface from ruvector-temporal-tensor for wasm32 targets.

pub use ruvector_temporal_tensor::ffi::*;
