// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! # Domain Module
//!
//! Core domain types following Domain-Driven Design principles.
//!
//! This module contains:
//! - **Entities**: Objects with identity that persist over time
//! - **Value Objects**: Immutable objects defined by their attributes
//! - **Domain Events**: Events that represent something that happened in the domain
//! - **Domain Errors**: Strongly-typed errors for domain operations

pub mod entities;
pub mod errors;
pub mod events;

pub use entities::*;
pub use errors::*;
pub use events::*;
