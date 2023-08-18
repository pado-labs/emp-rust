use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emp_tool::{prg::Prg, Block, Lpn};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lpn", move |bench| {
        let seed = Block::ZERO;
        let k = 588_160;
        let n = 10_000_000;
        let lpn = Lpn::<10>::new(seed, k);
        let mut x = vec![Block::ZERO; k as usize];
        let mut y = vec![Block::ZERO; n];
        let mut prg = Prg::new();
        prg.random_blocks(&mut x);
        prg.random_blocks(&mut y);
        bench.iter(|| {
            black_box(lpn.compute_naive::<4>(&mut y, &x));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
