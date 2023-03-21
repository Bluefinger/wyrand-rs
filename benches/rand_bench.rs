use criterion::{black_box, criterion_main, Criterion};
use rand_core::RngCore;
use wyrand::WyRand;

fn wyrand_benchmark(c: &mut Criterion) {
    c.bench_function("rand", |b| {
        let mut rng = WyRand::new(123456);

        b.iter(|| black_box(rng.rand()));
    });

    c.bench_function("next_64", |b| {
        let mut rng = WyRand::new(123456);

        b.iter(|| black_box(rng.next_u64()));
    });

    c.bench_function("fill_bytes", |b| {
        let mut rng = WyRand::new(123456);

        let data = [0u8; 2048];

        b.iter_batched_ref(
            || data,
            |data| {
                rng.fill_bytes(data);
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

pub fn benches() {
    let mut criterion: Criterion<_> = Criterion::default().configure_from_args();

    wyrand_benchmark(&mut criterion);
}

criterion_main!(benches);
