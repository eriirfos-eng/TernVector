// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! CLI command implementations for RuvLLM
//!
//! This module contains all the subcommand implementations:
//! - `download` - Download models from HuggingFace Hub
//! - `list` - List available and downloaded models
//! - `info` - Show detailed model information
//! - `serve` - Start an OpenAI-compatible inference server
//! - `chat` - Interactive chat mode
//! - `benchmark` - Run performance benchmarks
//! - `quantize` - Quantize models to GGUF format

pub mod benchmark;
pub mod chat;
pub mod download;
pub mod info;
pub mod list;
pub mod quantize;
pub mod serve;
