// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Temporary lib file to test dendrite module independently

pub mod dendrite;

pub use dendrite::{Compartment, Dendrite, DendriticTree, PlateauPotential};

#[derive(Debug, thiserror::Error)]
pub enum NervousSystemError {
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Compartment index out of bounds: {0}")]
    CompartmentOutOfBounds(usize),

    #[error("Synapse index out of bounds: {0}")]
    SynapseOutOfBounds(usize),
}

pub type Result<T> = std::result::Result<T, NervousSystemError>;
