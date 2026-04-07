// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! WebSocket module for real-time updates.
//!
//! This module provides WebSocket endpoints for:
//! - Real-time processing status updates
//! - Live cluster updates
//! - Streaming search results

pub mod handlers;

use axum::{routing::get, Router};

use crate::AppContext;

/// Create the WebSocket router.
#[must_use]
pub fn create_router(_ctx: AppContext) -> Router<AppContext> {
    Router::new()
        // Recording status updates
        .route("/recordings/:id", get(handlers::recording_status_ws))
        // All events stream (admin)
        .route("/events", get(handlers::events_ws))
}
