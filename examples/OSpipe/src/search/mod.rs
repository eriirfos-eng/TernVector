// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Query routing and hybrid search.
//!
//! Provides intelligent query routing that selects the optimal search
//! backend (semantic, keyword, temporal, graph, or hybrid) based on
//! query characteristics.

pub mod enhanced;
pub mod hybrid;
pub mod mmr;
pub mod reranker;
pub mod router;

pub use enhanced::EnhancedSearch;
pub use hybrid::HybridSearch;
pub use mmr::MmrReranker;
pub use reranker::AttentionReranker;
pub use router::{QueryRoute, QueryRouter};
