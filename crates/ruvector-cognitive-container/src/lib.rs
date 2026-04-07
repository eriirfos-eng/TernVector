// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Verifiable WASM cognitive container with canonical witness chains.
//!
//! This crate composes cognitive primitives (graph ingest, min-cut, spectral
//! analysis, evidence accumulation) into a sealed container that produces a
//! tamper-evident witness chain linking every epoch to its predecessor.

pub mod container;
pub mod epoch;
pub mod error;
pub mod memory;
pub mod witness;

pub use container::{
    CognitiveContainer, ComponentMask, ContainerConfig, ContainerSnapshot, Delta, TickResult,
};
pub use epoch::{ContainerEpochBudget, EpochController, Phase};
pub use error::{ContainerError, Result};
pub use memory::{Arena, MemoryConfig, MemorySlab};
pub use witness::{CoherenceDecision, ContainerWitnessReceipt, VerificationResult, WitnessChain};
