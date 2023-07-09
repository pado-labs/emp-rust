use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_tool::{prg::Prg, Block, ZERO_BLOCK};
use rand_core::RngCore;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Prg::bool", move |bench| {
        let mut prg = Prg::new();
        let mut x = false;
        bench.iter(|| {
            x = prg.random_bool();
            black_box(x)
        });
    });

    c.bench_function("Prg::bools", move |bench| {
        let mut prg = Prg::new();
        let mut x = [false; 10];
        bench.iter(|| {
            black_box(prg.random_bools(black_box(&mut x)));
        });
    });

    c.bench_function("Prg::byte", move |bench| {
        let mut prg = Prg::new();
        let mut x = 0u8;
        bench.iter(|| {
            x = prg.random_byte();
            black_box(x);
        });
    });

    c.bench_function("Prg::bytes", move |bench| {
        let mut prg = Prg::new();
        let mut x = (0..16 * 1024)
            .map(|_| rand::random::<u8>())
            .collect::<Vec<u8>>();
        bench.iter(|| {
            prg.fill_bytes(black_box(&mut x));
        });
    });

    c.bench_function("Prg::block", move |bench| {
        let mut prg = Prg::new();
        let mut x = ZERO_BLOCK;
        bench.iter(|| {
            x = prg.random_block();
            black_box(x);
        });
    });

    c.bench_function("Prg::blocks", move |bench| {
        let mut prg = Prg::new();
        let mut x = (0..16 * 1024)
            .map(|_| rand::random::<Block>())
            .collect::<Vec<Block>>();
        bench.iter(|| {
            prg.random_blocks(black_box(&mut x));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
