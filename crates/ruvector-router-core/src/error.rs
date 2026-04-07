// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Error types for the vector database

use thiserror::Error;

/// Result type alias for vector database operations
pub type Result<T> = std::result::Result<T, VectorDbError>;

/// Error types that can occur during vector database operations
#[derive(Error, Debug)]
pub enum VectorDbError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Index error
    #[error("Index error: {0}")]
    Index(String),

    /// Quantization error
    #[error("Quantization error: {0}")]
    Quantization(String),

    /// Invalid dimensions
    #[error("Invalid dimensions: expected {expected}, got {actual}")]
    InvalidDimensions {
        /// Expected dimensions
        expected: usize,
        /// Actual dimensions
        actual: usize,
    },

    /// Vector not found
    #[error("Vector not found: {0}")]
    NotFound(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(String),

    /// Invalid path error
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<redb::Error> for VectorDbError {
    fn from(err: redb::Error) -> Self {
        VectorDbError::Database(err.to_string())
    }
}

impl From<redb::DatabaseError> for VectorDbError {
    fn from(err: redb::DatabaseError) -> Self {
        VectorDbError::Database(err.to_string())
    }
}

impl From<redb::StorageError> for VectorDbError {
    fn from(err: redb::StorageError) -> Self {
        VectorDbError::Storage(err.to_string())
    }
}

impl From<redb::TransactionError> for VectorDbError {
    fn from(err: redb::TransactionError) -> Self {
        VectorDbError::Database(err.to_string())
    }
}

impl From<redb::TableError> for VectorDbError {
    fn from(err: redb::TableError) -> Self {
        VectorDbError::Database(err.to_string())
    }
}

impl From<redb::CommitError> for VectorDbError {
    fn from(err: redb::CommitError) -> Self {
        VectorDbError::Database(err.to_string())
    }
}
