// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Domain layer for the Analysis bounded context.
//!
//! Contains core domain entities, value objects, repository traits, and domain events.

pub mod entities;
pub mod events;
pub mod repository;
pub mod value_objects;

// Re-export commonly used types
pub use entities::*;
pub use events::*;
pub use repository::*;
pub use value_objects::*;
