// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Witness logging for audit trails and ReasoningBank integration
//!
//! Every inference produces a small witness bundle that records
//! what happened and enables verification and replay.

pub mod hash;
pub mod log;

// Re-export WitnessLog from types as the canonical location
pub use crate::types::WitnessLog;
pub use hash::{compute_witness_hash, verify_witness_hash};
pub use log::{WitnessAggregator, WitnessBuilder};
