// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! SONA: Self-Optimizing Neural Architecture for DAG Learning

mod engine;
mod ewc;
mod micro_lora;
mod reasoning_bank;
mod trajectory;

pub use engine::DagSonaEngine;
pub use ewc::{EwcConfig, EwcPlusPlus};
pub use micro_lora::{MicroLoRA, MicroLoRAConfig};
pub use reasoning_bank::{DagPattern, DagReasoningBank, ReasoningBankConfig};
pub use trajectory::{DagTrajectory, DagTrajectoryBuffer};
