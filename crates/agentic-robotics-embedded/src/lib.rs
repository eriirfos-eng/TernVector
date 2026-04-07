// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! ROS3 Embedded Systems Support
//!
//! Provides support for embedded systems using Embassy and RTIC


/// Embedded task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbeddedPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Embedded system configuration
#[derive(Debug, Clone)]
pub struct EmbeddedConfig {
    pub tick_rate_hz: u32,
    pub stack_size: usize,
}

impl Default for EmbeddedConfig {
    fn default() -> Self {
        Self {
            tick_rate_hz: 1000,
            stack_size: 4096,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_config() {
        let config = EmbeddedConfig::default();
        assert_eq!(config.tick_rate_hz, 1000);
        assert_eq!(config.stack_size, 4096);
    }
}
