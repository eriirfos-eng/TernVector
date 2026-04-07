// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Core types and traits for 7sense bioacoustic analysis.
//!
//! This crate provides foundational types shared across all bounded contexts:
//! - Entity identifiers (strongly-typed IDs)
//! - Value objects (GeoLocation, Timestamp, AudioMetadata)
//! - Common error types
//! - Domain entities and events

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod identifiers;
pub mod values;
pub mod error;
pub mod domain;

// Re-export commonly used types
pub use identifiers::*;
pub use values::*;
pub use error::*;
