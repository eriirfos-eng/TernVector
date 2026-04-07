// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Binary entrypoint for the RVF streaming server.

use clap::Parser;
use std::path::PathBuf;

use rvf_server::ServerConfig;

#[derive(Parser)]
#[command(
    name = "rvf-server",
    about = "TernVector Format TCP/HTTP streaming server"
)]
struct Cli {
    /// HTTP listen port
    #[arg(long, default_value_t = 8080)]
    port: u16,

    /// TCP streaming listen port
    #[arg(long, default_value_t = 9090)]
    tcp_port: u16,

    /// Path to the RVF data directory / file
    #[arg(long, default_value = "data.rvf")]
    data_dir: PathBuf,

    /// Vector dimension (used when creating a new store)
    #[arg(long, default_value_t = 128)]
    dimension: u16,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = ServerConfig {
        http_port: cli.port,
        tcp_port: cli.tcp_port,
        data_path: cli.data_dir,
        dimension: cli.dimension,
    };

    if let Err(e) = rvf_server::run(config).await {
        eprintln!("fatal: {e}");
        std::process::exit(1);
    }
}
