// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Vector-Graph Fusion Module
//!
//! Unified retrieval substrate combining vector similarity and graph relations
//! with minimum-cut brittleness detection for robust knowledge retrieval.

mod fusion_graph;
mod optimizer;
mod structural_monitor;

pub use fusion_graph::{
    EdgeOrigin, FusionConfig, FusionEdge, FusionGraph, FusionNode, FusionResult, RelationType,
};
pub use optimizer::{
    LearningGate, MaintenancePlan, MaintenanceTask, OptimizationResult, Optimizer, OptimizerAction,
};
pub use structural_monitor::{
    BrittlenessSignal, MonitorConfig as StructuralMonitorConfig, MonitorState, StructuralMonitor,
    Trigger, TriggerType,
};
