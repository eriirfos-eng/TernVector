// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! WASM utility helpers.

/// Set panic hook for better panic messages in the browser.
pub fn set_panic_hook() {
    // No-op if console_error_panic_hook is not available.
    // In production, add the crate and feature for better diagnostics.
}

/// Log a message to the browser console.
pub fn console_log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}
