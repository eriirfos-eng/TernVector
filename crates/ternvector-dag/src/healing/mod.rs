// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Self-Healing System for Neural DAG Learning

mod anomaly;
mod drift_detector;
mod index_health;
mod orchestrator;
mod strategies;

pub use anomaly::{Anomaly, AnomalyConfig, AnomalyDetector, AnomalyType};
pub use drift_detector::{DriftMetric, DriftTrend, LearningDriftDetector};
pub use index_health::{
    HealthStatus, IndexCheckResult, IndexHealth, IndexHealthChecker, IndexThresholds, IndexType,
};
pub use orchestrator::{HealingCycleResult, HealingOrchestrator};
pub use strategies::{
    CacheFlushStrategy, IndexRebalanceStrategy, PatternResetStrategy, RepairResult, RepairStrategy,
};
