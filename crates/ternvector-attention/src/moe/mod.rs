// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Mixture of Experts (MoE) attention mechanisms
//!
//! This module provides MoE attention where different inputs route to specialized experts.

pub mod expert;
pub mod moe_attention;
pub mod router;

pub use expert::{Expert, ExpertType, HyperbolicExpert, LinearExpert, StandardExpert};
pub use moe_attention::{MoEAttention, MoEConfig};
pub use router::{LearnedRouter, Router, TopKRouting};
