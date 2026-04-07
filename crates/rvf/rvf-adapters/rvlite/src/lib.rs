// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Lightweight embedded vector store adapter for the TernVector Format (RVF).
//!
//! **rvlite** provides a minimal, ergonomic API for embedded vector storage
//! using the RVF Core Profile. It is designed for resource-constrained
//! environments (WASM, edge, embedded) where a full-featured vector
//! database is unnecessary.
//!
//! # Design philosophy
//!
//! - **Simple**: No metadata, no filters, no namespaces. Just vectors with IDs.
//! - **Small**: Minimal dependency surface; only `rvf-runtime` and `rvf-types`.
//! - **Safe**: Dimension validation, proper error handling, no panics.
//!
//! # Quick start
//!
//! ```no_run
//! use rvf_adapter_rvlite::{RvliteCollection, RvliteConfig, RvliteMetric};
//!
//! let config = RvliteConfig::new("/tmp/my_vectors.rvf", 128)
//!     .with_metric(RvliteMetric::Cosine);
//!
//! let mut col = RvliteCollection::create(config).unwrap();
//!
//! col.add(1, &vec![0.1; 128]).unwrap();
//! col.add(2, &vec![0.2; 128]).unwrap();
//!
//! let results = col.search(&vec![0.1; 128], 5);
//! for m in &results {
//!     println!("id={} distance={:.4}", m.id, m.distance);
//! }
//!
//! col.close().unwrap();
//! ```

pub mod collection;
pub mod config;
pub mod error;

pub use collection::{CompactStats, Match, RvliteCollection};
pub use config::{RvliteConfig, RvliteMetric};
pub use error::{Result, RvliteError};
