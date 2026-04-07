// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Domain layer for the Vector Space bounded context.
//!
//! Contains:
//! - Entities: Core domain objects with identity
//! - Value Objects: Immutable objects defined by their attributes
//! - Repository Traits: Abstractions for persistence
//! - Domain Errors: Error types specific to this context

pub mod entities;
pub mod repository;
pub mod error;

pub use entities::*;
pub use repository::*;
pub use error::*;
