// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! # ruvector-attention SDK
//!
//! High-level, ergonomic APIs for building attention mechanisms.

pub mod builder;
pub mod pipeline;
pub mod presets;

pub use builder::{flash, multi_head, scaled_dot, AttentionBuilder, AttentionType};
pub use pipeline::{AttentionPipeline, NormType, PipelineStage};
pub use presets::{for_graphs, for_large_scale, for_sequences, AttentionPreset};
