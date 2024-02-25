use criterion::{black_box, criterion_main, Criterion};

fn wyrand_benchmark(c: &mut Criterion) {
    use rand_core::RngCore;
    use wyrand::WyRand;

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

#[cfg(feature = "wyhash")]
fn wyhash_benchmark(c: &mut Criterion) {
    use std::hash::Hasher;

    use criterion::BenchmarkId;
    use wyrand::WyHash;

    let test_cases: [&str; 7] = [
        "",
        "a",
        "abc",
        "message digest",
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    ];

    test_cases
        .into_iter()
        .enumerate()
        .for_each(|(seed, input)| {
            c.bench_with_input(
                BenchmarkId::new("Hash message of length", input.len()),
                &input.as_bytes(),
                |b, &input| {
                    b.iter(|| {
                        let mut hasher = WyHash::new_with_default_secret(black_box(seed as u64));

                        hasher.write(input);

                        hasher.finish()
                    });
                },
            );
        });
}

pub fn benches() {
    let mut criterion: Criterion<_> = Criterion::default().configure_from_args();

    wyrand_benchmark(&mut criterion);
    #[cfg(feature = "wyhash")]
    wyhash_benchmark(&mut criterion);
}

criterion_main!(benches);
