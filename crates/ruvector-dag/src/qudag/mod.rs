// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! QuDAG Integration - Quantum-Resistant Distributed Pattern Learning

mod client;
mod consensus;
pub mod crypto;
mod network;
mod proposal;
mod sync;
pub mod tokens;

pub use client::QuDagClient;
pub use consensus::{ConsensusResult, Vote};
pub use network::{NetworkConfig, NetworkStatus};
pub use proposal::{PatternProposal, ProposalStatus};
pub use sync::PatternSync;
pub use tokens::{
    GovernanceError, Proposal as GovProposal, ProposalStatus as GovProposalStatus, ProposalType,
    VoteChoice,
};
pub use tokens::{GovernanceSystem, RewardCalculator, StakingManager};
pub use tokens::{RewardClaim, RewardSource, StakeInfo, StakingError};
