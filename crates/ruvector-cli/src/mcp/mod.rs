// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Model Context Protocol (MCP) implementation for Ruvector

pub mod gnn_cache;
pub mod handlers;
pub mod protocol;
pub mod transport;

pub use gnn_cache::*;
pub use handlers::*;
pub use protocol::*;
pub use transport::*;
