// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Proof-system types.

/// Proof tier (P1, P2, P3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ProofTier {
    /// P1: Capability check (< 1 us).
    P1 = 1,
    /// P2: Policy validation (< 100 us).
    P2 = 2,
    /// P3: Deep proof (< 10 ms).
    P3 = 3,
}

/// Result of a proof verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofResult {
    /// Proof passed.
    Passed,
    /// Proof failed.
    Failed,
    /// Proof was escalated to a higher tier.
    Escalated,
}

/// A proof token that attests to a verified proof.
#[derive(Debug, Clone, Copy)]
pub struct ProofToken {
    /// The tier that was verified.
    pub tier: ProofTier,
    /// Epoch when the proof was generated.
    pub epoch: u32,
    /// Truncated hash of the proof payload.
    pub hash: u32,
}
