// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Benchmark the coherence EMA filter.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rvm_coherence::EmaFilter;

fn bench_ema_update(c: &mut Criterion) {
    c.bench_function("ema_filter_update", |b| {
        let mut filter = EmaFilter::new(2000);
        let mut sample = 0u16;
        b.iter(|| {
            sample = sample.wrapping_add(100);
            black_box(filter.update(sample));
        });
    });
}

criterion_group!(benches, bench_ema_update);
criterion_main!(benches);
