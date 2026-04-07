// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Integration tests for Prime-Radiant Coherence Engine
//!
//! This module contains integration tests organized by bounded context:
//!
//! - `graph_tests`: SheafGraph CRUD operations and dimension validation
//! - `coherence_tests`: Energy computation and incremental updates
//! - `governance_tests`: Policy bundles and witness chain integrity
//! - `gate_tests`: Compute ladder escalation and persistence detection

mod coherence_tests;
mod gate_tests;
mod governance_tests;
mod graph_tests;
