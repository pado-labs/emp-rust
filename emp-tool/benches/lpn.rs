use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emp_tool::{prg::Prg, Block, Lpn};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lpn-native", move |bench| {
        let seed = Block::ZERO;
        let k = 588_160;
        let n = 10_616_092;
        let lpn = Lpn::<10>::new(seed, k);
        let mut x = vec![Block::ZERO; k as usize];
        let mut y = vec![Block::ZERO; n];
        let mut prg = Prg::new();
        prg.random_blocks(&mut x);
        prg.random_blocks(&mut y);
        bench.iter(|| {
            black_box(lpn.compute_naive(&mut y, &x));
        });
    });

    c.bench_function("lpn-rayon", move |bench| {
        let seed = Block::ZERO;
        let k = 588_160;
        let n = 10_616_092;
        let lpn = Lpn::<10>::new(seed, k);
        let mut x = vec![Block::ZERO; k as usize];
        let mut y = vec![Block::ZERO; n];
        let mut prg = Prg::new();
        prg.random_blocks(&mut x);
        prg.random_blocks(&mut y);
        bench.iter(|| {
            black_box(lpn.compute(&mut y, &x));
        });
    });

    c.bench_function("lpn-rayon-custmized", move |bench| {
        let seed = Block::ZERO;
        let k = 588_160;
        let n = 10_616_092;
        let lpn = Lpn::<10>::new(seed, k);
        let mut x = vec![Block::ZERO; k as usize];
        let mut y = vec![Block::ZERO; n];
        let mut prg = Prg::new();
        prg.random_blocks(&mut x);
        prg.random_blocks(&mut y);
        bench.iter(|| {
            black_box(lpn.compute_with_customized_threads(&mut y, &x, 8));
        });
    });

    c.bench_function("lpn-custmized", move |bench| {
        let seed = Block::ZERO;
        let k = 588_160;
        let n = 10_616_092;
        let lpn = Lpn::<10>::new(seed, k);
        let mut x = vec![Block::ZERO; k as usize];
        let mut y = vec![Block::ZERO; n];
        let mut prg = Prg::new();
        prg.random_blocks(&mut x);
        prg.random_blocks(&mut y);
        bench.iter(|| {
            black_box(lpn.compute_with(&mut y, &x, 8));
        });
    });
}

// criterion_group!(benches, criterion_benchmark);
criterion_group! {
    name = lpn;
    config = Criterion::default().warm_up_time(Duration::from_millis(1000)).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(lpn);
