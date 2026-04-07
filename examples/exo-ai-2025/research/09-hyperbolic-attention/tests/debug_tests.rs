// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

use hyperbolic_attention::prelude::*;

#[test]
fn debug_curvature_scaling() {
    let x = vec![0.5, 0.0];
    let y = vec![1.0, 0.0];

    let d1 = poincare_distance(&x, &y, 1.0);
    let d2 = poincare_distance(&x, &y, 2.0);

    println!("Distance with K=1.0: {}", d1);
    println!("Distance with K=2.0: {}", d2);
    println!("d2 > d1: {}", d2 > d1);
}

#[test]
fn debug_exp_log() {
    let x = vec![0.1, 0.2];
    let y = vec![0.3, 0.1];
    let k = 1.0;

    println!("x: {:?}", x);
    println!("y: {:?}", y);

    let v = logarithmic_map(&x, &y, k);
    println!("log_x(y) = v: {:?}", v);

    let y_reconstructed = exponential_map(&x, &v, k);
    println!("exp_x(v) = y': {:?}", y_reconstructed);
    println!("Original y: {:?}", y);

    for (i, (orig, recon)) in y.iter().zip(&y_reconstructed).enumerate() {
        println!(
            "  y[{}]: {} vs {} (diff: {})",
            i,
            orig,
            recon,
            (orig - recon).abs()
        );
    }
}
