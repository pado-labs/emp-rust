use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_tool::{
    hash::{CcrHash, CrHash, TccrHash},
    Block, ZERO_BLOCK,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash::cr", move |bench| {
        let hash = CrHash::new();
        let i = rand::random::<Block>();
        bench.iter(|| {
            let z = hash.hash_block(black_box(i));
            black_box(z);
        });
    });

    c.bench_function("hash::cr_blocks::<8>", move |bench| {
        let hash = CrHash::new();
        bench.iter(|| {
            black_box(hash.hash_many_blocks::<8>([ZERO_BLOCK; 8]));
        });
    });

    c.bench_function("hash::ccr", move |bench| {
        let hash = CcrHash::new();
        let i = rand::random::<Block>();
        bench.iter(|| {
            let z = hash.hash_block(black_box(i));
            black_box(z);
        });
    });

    c.bench_function("hash::ccr_blocks::<8>", move |bench| {
        let hash = CcrHash::new();
        bench.iter(|| {
            black_box(hash.hash_many_blocks::<8>([ZERO_BLOCK; 8]));
        });
    });

    c.bench_function("hash::tccr", move |bench| {
        let hash = TccrHash::new();
        let x = rand::random::<Block>();
        let i = rand::random::<u64>();
        bench.iter(|| {
            let z = hash.hash_block(black_box(x), black_box(i));
            black_box(z);
        });
    });

    c.bench_function("hash::tccr_blocks::<8>", move |bench| {
        let hash = TccrHash::new();
        bench.iter(|| {
            black_box(hash.hash_many_blocks::<8>([ZERO_BLOCK; 8], [1; 8]));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
