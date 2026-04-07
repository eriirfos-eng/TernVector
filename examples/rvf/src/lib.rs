// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! RVF examples crate.
//!
//! This crate contains example binaries demonstrating usage of the RVF
//! (TernVector Format) crates. Run individual examples with:
//!
//! ```bash
//! cargo run --example basic_store
//! cargo run --example progressive_index
//! cargo run --example quantization
//! cargo run --example wire_format
//! cargo run --example crypto_signing
//! cargo run --example filtered_search
//! ```
//!
//! Solver integration examples (sublinear solver + RVF):
//!
//! ```bash
//! cargo run --example solver_witness        # convergence witness chains
//! cargo run --example sparse_matrix_store   # CSR sparse matrix storage
//! cargo run --example solver_benchmark      # benchmark result analysis
//! ```
