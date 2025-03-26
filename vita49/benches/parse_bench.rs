// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use vita49::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let data_json = include_str!("../tests/spectral_data_packet.json5");
    let data_packet: Vrt =
        serde_json5::from_str(&data_json).expect("failed to parse JSON into VRT");
    let data_vec = data_packet.to_bytes().unwrap();
    let context_json = include_str!("../tests/context_packet.json5");
    let context_packet: Vrt =
        serde_json5::from_str(&context_json).expect("failed to parse JSON into VRT");
    let context_vec = context_packet.to_bytes().unwrap();

    c.bench_function("Parse signal data", |p| {
        p.iter(|| Vrt::try_from(black_box(&data_vec[..])).unwrap())
    });
    c.bench_function("Parse context", |p| {
        p.iter(|| Vrt::try_from(black_box(&context_vec[..])).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
