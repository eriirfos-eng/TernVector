// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Witness record re-exports from rvm-types.
//!
//! The canonical `WitnessRecord` and `ActionKind` definitions live in
//! `rvm-types` so they can be shared across all RVM crates. This module
//! re-exports them for convenience.

pub use rvm_types::ActionKind;
pub use rvm_types::WitnessRecord;
