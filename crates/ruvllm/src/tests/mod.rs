// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Comprehensive test suite for RuvLLM
//!
//! This module organizes all unit tests for the RuvLLM crate.

mod activation_tests;
mod attention_tests;
mod generation_tests;
mod gguf_tests;
mod witness_log_tests;

// Basic lib configuration tests (moved from lib.rs)
use crate::RuvLLMConfig;

#[test]
fn test_config_default() {
    let config = RuvLLMConfig::default();
    assert_eq!(config.max_sessions, 1000);
    assert_eq!(config.embedding_dim, 768);
}
