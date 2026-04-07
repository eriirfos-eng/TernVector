// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! OSpipe adapter for the TernVector Format (RVF).
//!
//! Maps OSpipe's observation-state pipeline onto the RVF segment model:
//!
//! - **VEC_SEG**: State vector embeddings (screen, audio, UI observations)
//! - **META_SEG**: Observation metadata (app name, content type, timestamps)
//! - **JOURNAL_SEG**: Deletion records for expired observations
//!
//! The adapter bridges OSpipe's `StoredEmbedding` / `CapturedFrame` world
//! (UUID ids, chrono timestamps, JSON metadata) to RVF's u64-id,
//! field-based metadata model.

pub mod observation_store;
pub mod pipeline;

pub use observation_store::{ObservationMeta, RvfObservationStore};
pub use pipeline::{PipelineConfig, RvfPipelineAdapter};
