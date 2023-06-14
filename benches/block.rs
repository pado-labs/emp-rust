use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_rust::block::Block;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = ChaCha12Rng::from_entropy();
    let a: [u8; 16] = rng.gen();
    let b: [u8; 16] = rng.gen();
    let a = Block::new(&a);
    let b = Block::new(&b);

    let x: u128 = rng.gen();
    let y: u128 = rng.gen();

    c.bench_function("Block::clmul", move |bench| {
        bench.iter(|| {
            black_box(a.clmul(&b));
        });
    });

    c.bench_function("Block::xor", move |bench| {
        bench.iter(|| {
            black_box(a ^ b);
        });
    });

    c.bench_function("Block::default", move |bench| {
        bench.iter(|| {
            black_box(Block::default());
        });
    });

    c.bench_function("Block::u128_xor", move |bench| {
        bench.iter(|| {
            black_box(x ^ y);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
