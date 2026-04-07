// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! mcp-brain-server: Cloud Run backend for TernVector Shared Brain
//!
//! Provides REST API for storing, searching, voting, and managing shared knowledge.
//! Every piece of knowledge is an RVF cognitive container with witness chains,
//! Ed25519 signatures, and differential privacy proofs.

pub mod aggregate;
pub mod auth;
pub mod cognitive;
pub mod drift;
pub mod embeddings;
pub mod gcs;
pub mod graph;
pub mod pipeline;
pub mod ranking;
pub mod rate_limit;
pub mod reputation;
pub mod routes;
pub mod store;
pub mod tests;
pub mod midstream;
pub mod types;
pub mod trainer;
pub mod verify;
pub mod voice;
pub mod symbolic;
pub mod optimizer;
pub mod web_memory;
pub mod web_ingest;
pub mod web_store;
pub mod pubmed;
pub mod quantization;
pub mod notify;
pub mod gist;
