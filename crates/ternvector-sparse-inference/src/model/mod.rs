// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Model loading and inference infrastructure

pub mod gguf;
pub mod loader;
pub mod runners;
pub mod types;

pub use gguf::{GgufHeader, GgufModel, GgufParser, GgufTensorInfo, GgufTensorType, GgufValue};
pub use loader::{ModelArchitecture, ModelLoader, ModelMetadata, QuantizationType};
pub use runners::{
    BertModel, LFM2Model, LlamaLayer, LlamaMLP, LlamaModel, ModelRunner, SparseModel,
};
pub use types::{InferenceConfig, ModelInput, ModelOutput, Tensor};
