// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Local Swarm Example
//!
//! Demonstrates local shared-memory swarm communication.

use ruvector_edge::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    println!("Local Swarm Example");
    println!("==================\n");

    // Create coordinator
    let config = SwarmConfig::default()
        .with_agent_id("local-coordinator")
        .with_role(AgentRole::Coordinator);

    let coordinator = SwarmAgent::new(config).await?;
    println!("Created coordinator: {}", coordinator.id());

    // Create worker
    let worker_config = SwarmConfig::default()
        .with_agent_id("local-worker-1")
        .with_role(AgentRole::Worker);

    let worker = SwarmAgent::new(worker_config).await?;
    println!("Created worker: {}", worker.id());

    // Simulate learning
    worker.learn("local_task", "process_data", 0.9).await;
    println!("\nWorker learned: local_task -> process_data (0.9)");

    // Get best action
    if let Some((action, confidence)) = worker.get_best_action(
        "local_task",
        &["process_data".to_string(), "skip_data".to_string()]
    ).await {
        println!("Best action: {} (confidence: {:.1}%)", action, confidence * 100.0);
    }

    println!("\nLocal swarm example complete!");

    Ok(())
}
