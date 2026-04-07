// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Memory, power, and latency profiling for attention-mechanism benchmarks.

pub mod config_hash;
pub mod csv_emitter;
pub mod latency;
pub mod memory;
pub mod power;

pub use config_hash::{config_hash, BenchConfig};
pub use csv_emitter::{write_latency_csv, write_memory_csv, write_results_csv, ResultRow};
pub use latency::{compute_latency_stats, LatencyRecord, LatencyStats};
pub use memory::{capture_memory, MemoryReport, MemorySnapshot, MemoryTracker};
pub use power::{EnergyResult, MockPowerSource, PowerSample, PowerSource, PowerTracker};
