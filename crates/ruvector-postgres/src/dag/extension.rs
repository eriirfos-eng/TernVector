// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Extension initialization and SQL functions

use pgrx::prelude::*;
use super::guc;
use super::state::DAG_STATE;

/// Initialize the DAG extension
pub fn init() {
    // Initialize GUC variables
    guc::init_guc();

    // Register background worker
    super::worker::register_worker();
}

// SQL Functions

#[pg_extern]
fn ruvector_dag_set_enabled(enabled: bool) {
    if enabled {
        DAG_STATE.enable();
    } else {
        DAG_STATE.disable();
    }
}

#[pg_extern]
fn ruvector_dag_is_enabled() -> bool {
    DAG_STATE.is_enabled()
}

#[pg_extern]
fn ruvector_dag_status() -> pgrx::JsonB {
    let status = serde_json::json!({
        "enabled": DAG_STATE.is_enabled(),
        "pattern_count": DAG_STATE.get_pattern_count(),
        "trajectory_count": DAG_STATE.get_trajectory_count(),
        "learning_rate": guc::get_learning_rate(),
        "attention_mechanism": guc::get_attention_mechanism(),
    });

    pgrx::JsonB(status)
}

#[pg_extern]
fn ruvector_dag_set_learning_rate(rate: f64) {
    // Would update GUC variable
    // For now, just validate
    if rate < 0.0 || rate > 1.0 {
        pgrx::error!("Learning rate must be between 0 and 1");
    }
}

#[pg_extern]
fn ruvector_dag_set_attention(mechanism: &str) {
    let valid = ["topological", "causal_cone", "critical_path",
                 "mincut_gated", "hierarchical_lorentz",
                 "parallel_branch", "temporal_btsp", "auto"];

    if !valid.contains(&mechanism) {
        pgrx::error!("Invalid attention mechanism: {}. Valid: {:?}", mechanism, valid);
    }
}
