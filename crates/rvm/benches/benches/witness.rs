// SPDX-License-Identifier: LicenseRef-BSL-1.1
// RFI-IRFOS Resonant Edge Suite — TernVector
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// This software is licensed under the Business Source License 1.1 until 2030-04-03.
// See LICENSE-BSL in the repository root for details.

//! Benchmark witness log operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rvm_types::WitnessRecord;
use rvm_witness::WitnessLog;

fn bench_witness_append(c: &mut Criterion) {
    c.bench_function("witness_log_append_256", |b| {
        let mut log = WitnessLog::<256>::new();
        b.iter(|| {
            black_box(log.append(WitnessRecord::zeroed()));
        });
    });
}

criterion_group!(benches, bench_witness_append);
criterion_main!(benches);
