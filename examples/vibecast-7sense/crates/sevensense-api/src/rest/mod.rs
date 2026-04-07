// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! REST API module for 7sense.
//!
//! This module provides RESTful endpoints for:
//! - Recording upload and management
//! - Segment similarity search
//! - Cluster discovery and labeling
//! - Evidence pack retrieval
//!
//! ## API Versioning
//!
//! All endpoints are versioned under `/api/v1/`. Breaking changes will
//! result in a new API version (e.g., `/api/v2/`).
//!
//! ## Authentication
//!
//! If `SEVENSENSE_API_KEY` is set, all requests must include an
//! `Authorization: Bearer <api_key>` header.

pub mod handlers;
pub mod middleware;
pub mod routes;

pub use handlers::*;
pub use routes::create_router;
