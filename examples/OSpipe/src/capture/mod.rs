// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Capture module for processing screen, audio, and UI event data.
//!
//! This module defines the data structures that represent captured frames
//! from Screenpipe sources: OCR text from screen recordings, audio
//! transcriptions, and UI accessibility events.

pub mod frame;

pub use frame::{CaptureSource, CapturedFrame, FrameContent, FrameMetadata};
