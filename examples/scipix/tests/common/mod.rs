// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

// Common test utilities
//
// Provides shared functionality for integration tests

pub mod images;
pub mod latex;
pub mod metrics;
pub mod server;
pub mod types;

// Re-export commonly used types and functions
pub use images::{generate_fraction, generate_integral, generate_simple_equation, generate_symbol};
pub use latex::{calculate_similarity, expressions_match, normalize};
pub use metrics::{calculate_bleu, calculate_cer, calculate_wer};
pub use server::TestServer;
pub use types::{CacheStats, OutputFormat, ProcessingOptions, ProcessingResult};
