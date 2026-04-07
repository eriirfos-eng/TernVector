// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Redundancy Suggestions for reliability

use super::bottleneck::Bottleneck;
use crate::dag::{OperatorType, QueryDag};

/// Suggestion for adding redundancy
#[derive(Debug, Clone)]
pub struct RedundancySuggestion {
    pub target_node: usize,
    pub strategy: RedundancyStrategy,
    pub expected_improvement: f64,
    pub cost_increase: f64,
}

#[derive(Debug, Clone)]
pub enum RedundancyStrategy {
    /// Duplicate the node's computation
    Replicate,
    /// Add alternative path
    AlternativePath,
    /// Cache intermediate results
    Materialize,
    /// Pre-compute during idle time
    Prefetch,
}

impl RedundancySuggestion {
    pub fn generate(dag: &QueryDag, bottlenecks: &[Bottleneck]) -> Vec<Self> {
        let mut suggestions = Vec::new();

        for bottleneck in bottlenecks {
            let node = dag.get_node(bottleneck.node_id);
            if node.is_none() {
                continue;
            }
            let node = node.unwrap();

            // Determine best strategy based on operator type
            let strategy = match &node.op_type {
                OperatorType::SeqScan { .. }
                | OperatorType::IndexScan { .. }
                | OperatorType::IvfFlatScan { .. } => RedundancyStrategy::Materialize,
                OperatorType::HnswScan { .. } => RedundancyStrategy::Prefetch,
                _ => RedundancyStrategy::Replicate,
            };

            suggestions.push(RedundancySuggestion {
                target_node: bottleneck.node_id,
                strategy,
                expected_improvement: bottleneck.impact_estimate * 0.3,
                cost_increase: node.estimated_cost * 0.1,
            });
        }

        suggestions
    }
}
