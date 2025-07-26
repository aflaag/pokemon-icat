use criterion::{criterion_group, criterion_main};

fn from_rgb(c: &mut criterion::Criterion) {
    c.bench_function("convert from True Colour", move |b| {
        b.iter(|| {
            for rgb in 0..(1 << 24) {
                criterion::black_box(ansi_colours::ansi256_from_rgb(rgb));
            }
        })
    });
}

fn to_rgb(c: &mut criterion::Criterion) {
    c.bench_function("convert to True Colour", move |b| {
        b.iter(|| {
            for idx in 0..256 {
                criterion::black_box(ansi_colours::rgb_from_ansi256(idx as u8));
            }
        })
    });
}

criterion_group!(benches, from_rgb, to_rgb);
criterion_main!(benches);
