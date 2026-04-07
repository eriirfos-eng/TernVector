// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Domain layer for the audio ingestion bounded context.
//!
//! This module contains the core domain model:
//! - Entities: Recording, CallSegment
//! - Value objects: SignalQuality
//! - Repository traits: RecordingRepository

pub mod entities;
pub mod repository;

pub use entities::*;
pub use repository::*;
