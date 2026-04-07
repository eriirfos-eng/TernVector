// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Autonomous Economics for edge-net P2P Network
//!
//! This module provides economic mechanisms for the compute marketplace:
//!
//! ## Components
//!
//! - **AMM**: Automated Market Maker for compute pricing
//!   - x * y = k invariant
//!   - Dynamic fee based on utilization
//!   - Liquidity provision
//!   - Accessibility helpers for cost estimation and price transparency
//!
//! - **Reputation**: Bonding curves for trust and pricing
//!   - Reputation-weighted discounts
//!   - Superlinear task allocation priority
//!   - Stake requirements
//!   - Newcomer tier for zero-barrier onboarding
//!
//! - **Brain Rewards**: Brain-specific reward schedule
//!   - Free-to-read, earn-to-write model
//!   - Halving schedule for sustainability
//!   - Epoch-bounded minting budget

pub mod amm;
pub mod reputation;
pub mod brain_rewards;

pub use amm::*;
pub use reputation::*;
pub use brain_rewards::*;
