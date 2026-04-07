// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! AArch64-specific HAL implementation for the RVM microhypervisor.
//!
//! This module provides bare-metal support for QEMU virt (AArch64):
//! - EL2 boot assembly stubs
//! - Stage-2 page table management
//! - PL011 UART driver
//! - GICv2 interrupt controller
//! - ARM generic timer
//!
//! This is the ONE crate in RVM where `unsafe` is permitted, because it
//! forms the hardware boundary. Every `unsafe` block has a `// SAFETY:`
//! comment documenting the invariant.

pub mod boot;
pub mod interrupts;
pub mod mmu;
pub mod timer;
pub mod uart;
