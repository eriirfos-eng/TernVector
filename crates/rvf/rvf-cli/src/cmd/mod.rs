// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

pub mod compact;
pub mod create;
pub mod delete;
pub mod derive;
pub mod embed_ebpf;
pub mod embed_kernel;
pub mod filter;
pub mod freeze;
pub mod ingest;
pub mod inspect;
pub mod launch;
pub mod query;
pub mod rebuild_refcounts;
pub mod serve;
pub mod status;
pub mod verify_attestation;
pub mod verify_witness;

/// Convert an RvfError into a boxed std::error::Error.
///
/// RvfError implements Display but not std::error::Error (it is no_std),
/// so we wrap it in a std::io::Error for CLI error propagation.
pub fn map_rvf_err(e: rvf_types::RvfError) -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::other(format!("{e}")))
}
