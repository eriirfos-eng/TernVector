// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! IndexedDB storage backend for WASM persistence
//!
//! Provides async-compatible persistence using IndexedDB for:
//! - Vector database state
//! - Cypher graph state
//! - SPARQL triple store state

pub mod indexeddb;
pub mod state;

#[cfg(feature = "rvf-backend")]
pub mod epoch;

#[cfg(feature = "rvf-backend")]
pub mod writer_lease;

#[cfg(feature = "rvf-backend")]
pub mod id_map;

pub use indexeddb::IndexedDBStorage;
pub use state::{GraphState, RvLiteState, TripleStoreState, VectorState};
