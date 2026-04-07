// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Error types for WASM embeddings

use thiserror::Error;
use wasm_bindgen::prelude::*;

/// Result type for WASM embedding operations
pub type Result<T> = std::result::Result<T, WasmEmbeddingError>;

/// Errors that can occur during WASM embedding operations
#[derive(Error, Debug)]
pub enum WasmEmbeddingError {
    #[error("Model error: {0}")]
    Model(String),

    #[error("Tokenizer error: {0}")]
    Tokenizer(String),

    #[error("Inference error: {0}")]
    Inference(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl WasmEmbeddingError {
    pub fn model(msg: impl Into<String>) -> Self {
        Self::Model(msg.into())
    }

    pub fn tokenizer(msg: impl Into<String>) -> Self {
        Self::Tokenizer(msg.into())
    }

    pub fn inference(msg: impl Into<String>) -> Self {
        Self::Inference(msg.into())
    }

    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::InvalidInput(msg.into())
    }
}

impl From<WasmEmbeddingError> for JsValue {
    fn from(err: WasmEmbeddingError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

impl From<tract_onnx::prelude::TractError> for WasmEmbeddingError {
    fn from(err: tract_onnx::prelude::TractError) -> Self {
        Self::Model(err.to_string())
    }
}

impl From<serde_json::Error> for WasmEmbeddingError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}
