// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! SONA Learning Loops
//!
//! Three-tier temporal learning architecture:
//! - Loop A (Instant): Per-request trajectory recording and micro-LoRA updates
//! - Loop B (Background): Hourly pattern extraction and base LoRA updates
//! - Loop C (Deep): Weekly dream consolidation and full EWC++ update

pub mod background;
pub mod coordinator;
pub mod instant;

pub use background::BackgroundLoop;
pub use coordinator::LoopCoordinator;
pub use instant::InstantLoop;
