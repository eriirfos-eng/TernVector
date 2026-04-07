// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

fn main() {
    // Emit cfg flags for SIMD detection at build time
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(target_arch = "x86_64")]
    {
        if std::env::var("CARGO_CFG_TARGET_FEATURE").map_or(false, |f| f.contains("avx2")) {
            println!("cargo:rustc-cfg=has_avx2");
        }
        if std::env::var("CARGO_CFG_TARGET_FEATURE").map_or(false, |f| f.contains("avx512f")) {
            println!("cargo:rustc-cfg=has_avx512");
        }
    }
}
