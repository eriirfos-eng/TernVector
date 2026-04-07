// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

// SQL query engine module for rvlite
// Provides SQL interface for vector database operations with WASM compatibility

mod ast;
mod executor;
mod parser;

pub use ast::*;
pub use executor::{ExecutionResult, SqlEngine};
pub use parser::{ParseError, SqlParser};

#[cfg(test)]
mod tests;
