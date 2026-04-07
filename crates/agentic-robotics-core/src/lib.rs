// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! ROS3 Core - Next-generation Robot Operating System
//!
//! A ground-up Rust rewrite of ROS targeting microsecond-scale determinism
//! with hybrid WASM/native deployment via npm.

pub mod middleware;
pub mod serialization;
pub mod message;
pub mod publisher;
pub mod subscriber;
pub mod service;
pub mod error;

pub use middleware::Zenoh;
pub use message::{Message, RobotState, PointCloud};
pub use publisher::Publisher;
pub use subscriber::Subscriber;
pub use service::{Service, Queryable};
pub use error::{Result, Error};

/// ROS3 Core version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize ROS3 runtime
pub fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    tracing::info!("ROS3 Core v{} initialized", VERSION);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let result = init();
        assert!(result.is_ok());
    }
}
