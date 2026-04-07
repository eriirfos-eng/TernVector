// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! rvAgent ACP — Agent Communication Protocol server.
//!
//! Provides an axum-based HTTP server implementing the ACP protocol
//! with authentication, rate limiting, and body size enforcement
//! per ADR-099 and ADR-103 C6.

pub mod agent;
pub mod auth;
pub mod server;
pub mod types;
