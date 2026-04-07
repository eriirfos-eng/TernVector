// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

// Time Crystal Cognition Library
// Cognitive Time Crystals: Discrete Time Translation Symmetry Breaking in Working Memory

pub mod discrete_time_crystal;
pub mod floquet_cognition;
pub mod simd_optimizations;
pub mod temporal_memory;

// Re-export main types
pub use discrete_time_crystal::{DTCConfig, DiscreteTimeCrystal};
pub use floquet_cognition::{
    FloquetCognitiveSystem, FloquetConfig, FloquetTrajectory, PhaseDiagram,
};
pub use simd_optimizations::{
    HierarchicalTimeCrystal, SimdDTC, SimdFloquet, TopologicalTimeCrystal,
};
pub use temporal_memory::{
    MemoryItem, MemoryStats, TemporalMemory, TemporalMemoryConfig, WorkingMemoryTask,
};
