// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! SONA (Self-Optimizing Neural Architecture)
//!
//! Adaptive learning system with ReasoningBank integration.

pub mod types;
pub mod lora;
pub mod trajectory;
pub mod ewc;
pub mod reasoning_bank;
pub mod loops;
pub mod engine;

// Re-export main types
pub use types::{
    LearningSignal, QueryTrajectory, TrajectoryStep,
    LearnedPattern, PatternType, SignalMetadata, SonaConfig,
};
pub use lora::{MicroLoRA, BaseLoRA, LoRAEngine, LoRALayer};
pub use trajectory::{TrajectoryBuffer, TrajectoryBuilder, TrajectoryIdGen};
pub use ewc::{EwcConfig, EwcPlusPlus, TaskFisher};
pub use reasoning_bank::{ReasoningBank, PatternConfig};
pub use loops::{InstantLoop, BackgroundLoop, LoopCoordinator};
pub use engine::SonaEngine;
