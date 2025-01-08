// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use vita49::Vrt;

fn criterion_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../tests/spectral_data_packet.vrt");
    let data_vec = data.to_vec();
    let context = include_bytes!("../tests/context_packet.vrt");
    let context_vec = context.to_vec();
    c.bench_function("Parse signal data", |p| {
        p.iter(|| Vrt::try_from(black_box(&data_vec[..])).unwrap())
    });
    c.bench_function("Parse context", |p| {
        p.iter(|| Vrt::try_from(black_box(&context_vec[..])).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
