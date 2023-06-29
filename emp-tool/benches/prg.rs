use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_tool::prg::Prg;
use rand_core::RngCore;

fn criterion_benchmark(c: &mut Criterion) {
    let mut prg = Prg::new();
    let mut x = (0..16 * 1024)
        .map(|_| rand::random::<u8>())
        .collect::<Vec<u8>>();
    c.bench_function("Prg::rand", move |bench| {
        bench.iter(|| {
            prg.fill_bytes(black_box(&mut x));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
