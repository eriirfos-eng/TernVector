// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Foreign function interfaces for FPGA Transformer
//!
//! Provides C ABI and WASM bindings.

#[cfg(feature = "wasm")]
pub mod wasm_bindgen;

pub mod c_abi;
