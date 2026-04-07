// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

use wasm_bindgen::prelude::*;

pub mod attention;
pub mod training;
pub mod utils;

/// Initialize the WASM module with panic hook
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Get the version of the ruvector-attention-wasm crate
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get information about available attention mechanisms
#[wasm_bindgen]
pub fn available_mechanisms() -> JsValue {
    let mechanisms = vec![
        "scaled_dot_product",
        "multi_head",
        "hyperbolic",
        "linear",
        "flash",
        "local_global",
        "moe",
    ];
    serde_wasm_bindgen::to_value(&mechanisms).unwrap()
}
