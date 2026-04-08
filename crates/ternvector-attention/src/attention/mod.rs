// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Attention mechanism implementations.
//!
//! This module provides concrete implementations of various attention mechanisms
//! including scaled dot-product attention and multi-head attention.

pub mod flash;
pub mod kv_cache;
pub mod mla;
pub mod multi_head;
pub mod scaled_dot_product;
pub mod speculative;
pub mod ssm;

pub use flash::{
    causal_block_mask, FlashAttention3, FlashConfig, FlashOutput, IOStats, RingAttention,
    RingDeviceOutput,
};
pub use mla::{MLACache, MLAConfig, MLALayer, MemoryComparison};
pub use multi_head::MultiHeadAttention;
pub use scaled_dot_product::ScaledDotProductAttention;
pub use speculative::{
    medusa_decode, theoretical_speedup, AcceptedTokens, DecodingStats, DraftModel, MedusaHead,
    MedusaResult, SimpleDraftModel, SimpleMedusaHead, SimpleTargetModel, SpeculativeConfig,
    SpeculativeDecoder, TargetModel, TokenId,
};
pub use ssm::{
    HybridBlock, HybridConfig, LayerKind, MambaBlock, SSMConfig, SSMState, SelectiveSSM,
};
