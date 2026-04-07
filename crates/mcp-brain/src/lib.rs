// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! mcp-brain: MCP server for the TernVector Shared Brain
//!
//! Enables Claude Code sessions to share and discover learning across sessions.
//! Knowledge is stored as RVF cognitive containers with witness chains,
//! Ed25519 signatures, and differential privacy proofs.
//!
//! # MCP Tools (10)
//!
//! - **brain_share**: Share a learning with the collective
//! - **brain_search**: Semantic search across shared knowledge
//! - **brain_get**: Retrieve a specific memory with full provenance
//! - **brain_vote**: Quality-gate a memory (Bayesian update)
//! - **brain_transfer**: Apply learned priors cross-domain
//! - **brain_drift**: Check if shared knowledge has drifted
//! - **brain_partition**: Get knowledge partitioned by mincut topology
//! - **brain_list**: List recent memories by category/quality
//! - **brain_delete**: Delete own contribution
//! - **brain_status**: System health
//!
//! # Usage
//!
//! ```no_run
//! use mcp_brain::McpBrainServer;
//!
//! #[tokio::main]
//! async fn main() {
//!     let server = McpBrainServer::new();
//!     server.run_stdio().await.expect("Server failed");
//! }
//! ```

pub mod client;
pub mod embed;
pub mod pipeline;
pub mod server;
pub mod tools;
pub mod types;

pub use server::McpBrainServer;
