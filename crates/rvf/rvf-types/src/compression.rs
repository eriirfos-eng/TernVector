// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Compression algorithm identifiers.

/// Identifies the compression algorithm applied to a segment payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum CompressionAlgo {
    /// No compression.
    None = 0,
    /// LZ4 block compression (~4 GB/s decompress).
    Lz4 = 1,
    /// Zstandard compression (~1.5 GB/s decompress, higher ratio).
    Zstd = 2,
    /// Domain-specific custom compression.
    Custom = 3,
}

impl TryFrom<u8> for CompressionAlgo {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Lz4),
            2 => Ok(Self::Zstd),
            3 => Ok(Self::Custom),
            other => Err(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        for raw in 0..=3u8 {
            let algo = CompressionAlgo::try_from(raw).unwrap();
            assert_eq!(algo as u8, raw);
        }
    }

    #[test]
    fn invalid_value() {
        assert_eq!(CompressionAlgo::try_from(4), Err(4));
    }
}
