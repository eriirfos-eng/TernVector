// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Common error types for 7sense.

/// Domain-level errors.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    /// Invalid configuration.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Entity not found.
    #[error("Not found: {entity_type} with id {id}")]
    NotFound {
        /// Entity type.
        entity_type: &'static str,
        /// Entity ID.
        id: String,
    },
    /// Validation error.
    #[error("Validation failed: {0}")]
    Validation(String),
}

impl CoreError {
    /// Creates a NotFound error.
    pub fn not_found(entity_type: &'static str, id: impl ToString) -> Self {
        Self::NotFound {
            entity_type,
            id: id.to_string(),
        }
    }

    /// Creates a Validation error.
    pub fn validation(message: impl ToString) -> Self {
        Self::Validation(message.to_string())
    }
}
