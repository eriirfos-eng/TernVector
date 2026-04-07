// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! MCP Brain server binary
//!
//! Runs the MCP Brain server on stdio for integration with Claude Code.

use mcp_brain::McpBrainServer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(filter)
        .init();

    let server = McpBrainServer::new();

    tracing::info!("MCP Brain server v{} starting", env!("CARGO_PKG_VERSION"));
    tracing::info!("Backend: brain.ruv.io");

    server.run_stdio().await?;

    Ok(())
}
