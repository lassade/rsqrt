#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate criterion;

use core::time::Duration;
use criterion::{black_box, BatchSize, BenchmarkGroup, Criterion, Throughput};
use rand::prelude::*;
use std::fmt;

const WARM_UP_TIME: Duration = Duration::from_secs(5);
const MEASUREMENT_TIME: Duration = Duration::from_secs(55);

use rsqrt::*;

#[repr(align(16))]
struct Wrapper(f32);

fn rsqrt(c: &mut Criterion) {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(core_ids[0]);

    // make sure the test value is aligned for the SEE instructions
    let v = Wrapper(80.0);

    let mut group = c.benchmark_group("rsqrt");
    group.bench_with_input("fpu", &v, |b, w| b.iter(|| black_box(fpu_rsqrt(w.0))));
    group.bench_with_input("quake3", &v, |b, w| b.iter(|| black_box(quake3_rsqrt(w.0))));
    group.bench_with_input("sse", &v, |b, w| b.iter(|| black_box(see_rsqrt(w.0))));
    group.bench_with_input("sse_nr1", &v, |b, w| {
        b.iter(|| black_box(see_rsqrt_nr1(w.0)))
    });
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.finish();
}

criterion_group!(benches, rsqrt);
criterion_main!(benches);
