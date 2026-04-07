// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Temperature-tiered vector quantization for the TernVector Format (RVF).
//!
//! Provides three quantization levels mapped to temperature tiers:
//!
//! | Tier | Quantization | Compression |
//! |------|-------------|-------------|
//! | Hot  | Scalar (int8) | 4x |
//! | Warm | Product (PQ)  | 8-16x |
//! | Cold | Binary (1-bit)| 32x |
//!
//! A Count-Min Sketch tracks per-block access frequency to drive
//! promotion/demotion decisions.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod binary;
pub mod codec;
pub mod product;
pub mod scalar;
pub mod sketch;
pub mod tier;
pub mod traits;

pub use binary::{decode_binary, encode_binary, hamming_distance};
pub use product::ProductQuantizer;
pub use scalar::ScalarQuantizer;
pub use sketch::CountMinSketch;
pub use tier::TemperatureTier;
pub use traits::Quantizer;
