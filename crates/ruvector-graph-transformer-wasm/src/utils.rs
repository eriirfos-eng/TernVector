// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! WASM utility helpers.

/// Set panic hook for better panic messages in the browser.
///
/// No-op; add `console_error_panic_hook` as an optional dependency for
/// improved browser diagnostics.
pub fn set_panic_hook() {
    // Intentional no-op. In production, wire up console_error_panic_hook.
}
