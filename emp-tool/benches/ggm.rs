use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emp_tool::{ggm_tree::GgmTree, Block};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ggm::gen::16M", move |bench| {
        let depth = 24;
        let ggm = GgmTree::new(depth);
        let mut tree = vec![Block::ZERO; 1 << depth];
        let mut k0 = vec![Block::ZERO; depth];
        let mut k1 = vec![Block::ZERO; depth];
        let seed = rand::random::<Block>();
        bench.iter(|| {
            black_box(ggm.gen(
                black_box(seed),
                black_box(&mut tree),
                black_box(&mut k0),
                black_box(&mut k1),
            ));
        });
    });

    c.bench_function("ggm::reconstruction::16M", move |bench| {
        let depth = 24;
        let ggm = GgmTree::new(depth);
        let mut tree = vec![Block::ZERO; 1 << (depth)];
        let k = vec![Block::ZERO; depth];
        let alpha = vec![false; depth];
        bench.iter(|| {
            black_box(ggm.reconstruct(black_box(&alpha), black_box(&k), black_box(&mut tree)))
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
