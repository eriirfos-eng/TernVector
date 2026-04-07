// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Advanced Optimizations for ESP32
//!
//! - Binary quantization (32x compression)
//! - Product quantization (8-32x compression)
//! - Lookup tables (fixed-point softmax)
//! - MicroLoRA (on-device adaptation)
//! - Sparse attention patterns
//! - MinCut-inspired pruning

pub mod binary_quant;
pub mod product_quant;
pub mod lookup_tables;
pub mod micro_lora;
pub mod sparse_attention;
pub mod pruning;

pub use binary_quant::{BinaryVector, BinaryEmbedding, hamming_distance, hamming_similarity, popcount8};
pub use product_quant::{ProductQuantizer, PQCode, PQConfig, PQDistanceTable};
pub use lookup_tables::{SoftmaxLUT, ExpLUT, DistanceLUT, SOFTMAX_LUT, EXP_LUT, DISTANCE_LUT};
pub use micro_lora::{MicroLoRA, LoRAConfig, LoRAStack};
pub use sparse_attention::{SparseAttention, AttentionPattern, AttentionPatternCache};
pub use pruning::{LayerPruner, PruningConfig, PruningMask, PruningStats, MinCutScorer};
