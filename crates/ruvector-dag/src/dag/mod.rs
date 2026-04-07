// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Core DAG data structures and algorithms

mod operator_node;
mod query_dag;
mod serialization;
mod traversal;

pub use operator_node::{OperatorNode, OperatorType};
pub use query_dag::{DagError, QueryDag};
pub use serialization::{DagDeserializer, DagSerializer};
pub use traversal::{BfsIterator, DfsIterator, TopologicalIterator};
