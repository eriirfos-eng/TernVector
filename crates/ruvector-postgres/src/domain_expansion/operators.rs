// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! PostgreSQL operator functions for domain expansion.

use pgrx::prelude::*;
use pgrx::JsonB;

use ruvector_domain_expansion::{ArmId, ContextBucket, DomainId, Solution};

use super::get_or_create_engine;

/// Perform cross-domain transfer learning.
#[pg_extern]
pub fn ruvector_domain_transfer(
    embeddings_json: JsonB,
    target_domain: &str,
    config_json: default!(JsonB, "JsonB(serde_json::json!({}))"),
) -> JsonB {
    let engine_lock = get_or_create_engine("default");
    let mut engine = engine_lock.write();

    let source_domain = config_json
        .0
        .get("source_domain")
        .and_then(|v| v.as_str())
        .unwrap_or("rust_synthesis");

    let source_id = DomainId(source_domain.to_string());
    let target_id = DomainId(target_domain.to_string());

    // Initiate transfer
    engine.initiate_transfer(&source_id, &target_id);

    // Embed input data
    let content = serde_json::to_string(&embeddings_json.0).unwrap_or_default();
    let solution = Solution {
        task_id: "transfer_input".to_string(),
        content,
        data: embeddings_json.0.clone(),
    };

    let embedding = engine.embed(&target_id, &solution);

    let domains = engine.domain_ids();

    JsonB(serde_json::json!({
        "status": "transfer_initiated",
        "source": source_domain,
        "target": target_domain,
        "embedding_dim": embedding.as_ref().map(|e| e.dim).unwrap_or(0),
        "available_domains": domains.iter().map(|d| &d.0).collect::<Vec<_>>(),
    }))
}
