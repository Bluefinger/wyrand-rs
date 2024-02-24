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

#[cfg(feature = "wyhash")]
fn wyhash_benchmark(c: &mut Criterion) {
    use std::hash::Hasher;

    use wyrand::WyHash;

    #[rustfmt::skip]
    let test_cases: [(u64, &str); 7] = [
        (0x0409_638e_e2bd_e459, ""),
        (0xa841_2d09_1b5f_e0a9, "a"),
        (0x32dd_92e4_b291_5153, "abc"),
        (0x8619_1240_89a3_a16b, "message digest"),
        (0x7a43_afb6_1d7f_5f40, "abcdefghijklmnopqrstuvwxyz"),
        (0xff42_329b_90e5_0d58, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
        (0xc39c_ab13_b115_aad3, "12345678901234567890123456789012345678901234567890123456789012345678901234567890"),
    ];

    test_cases
        .into_iter()
        .enumerate()
        .for_each(|(seed, (hash, input))| {
            let test_name = format!("Hash message of length: {}", input.len());

            c.bench_function(&test_name, |b| {
                b.iter(|| {
                    let mut hasher = black_box(WyHash::new_with_default_secret(seed as u64));

                    hasher.write(input.as_bytes());

                    assert_eq!(hash, hasher.finish())
                });
            });
        });
}

pub fn benches() {
    let mut criterion: Criterion<_> = Criterion::default().configure_from_args();

    wyrand_benchmark(&mut criterion);
    #[cfg(feature = "wyhash")]
    wyhash_benchmark(&mut criterion);
}

criterion_main!(benches);
