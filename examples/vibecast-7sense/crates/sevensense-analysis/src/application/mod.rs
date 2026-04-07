// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Application layer for the Analysis bounded context.
//!
//! Contains application services that orchestrate domain operations
//! and coordinate with infrastructure components.

pub mod services;

// Re-export service types
pub use services::{
    AnomalyDetectionService, ClusteringService, MotifDetectionService, SequenceAnalysisService,
};
