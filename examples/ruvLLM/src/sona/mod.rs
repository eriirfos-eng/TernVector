// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! SONA (Self-Optimizing Neural Architecture)
//!
//! Adaptive learning system with ReasoningBank integration.

pub mod engine;
pub mod ewc;
pub mod loops;
pub mod lora;
pub mod reasoning_bank;
pub mod trajectory;
pub mod types;

// Re-export main types
pub use engine::SonaEngine;
pub use ewc::{EwcConfig, EwcPlusPlus, TaskFisher};
pub use loops::{BackgroundLoop, InstantLoop, LoopCoordinator};
pub use lora::{BaseLoRA, LoRAEngine, LoRALayer, MicroLoRA};
pub use reasoning_bank::{PatternConfig, ReasoningBank};
pub use trajectory::{TrajectoryBuffer, TrajectoryBuilder, TrajectoryIdGen};
pub use types::{
    LearnedPattern, LearningSignal, PatternType, QueryTrajectory, SignalMetadata, SonaConfig,
    TrajectoryStep,
};
