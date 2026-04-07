// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Integration modules for Ruvector and RuvLLM ecosystems
//!
//! This module provides seamless integration with the Ruvector vector database
//! and RuvLLM language model inference framework.

pub mod ruvector;
pub mod ruvllm;

pub use ruvector::SparseEmbeddingProvider;
pub use ruvllm::SparseInferenceBackend;
