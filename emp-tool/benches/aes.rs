use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_tool::{block::Block, aes::Aes};

fn criterion_benchmark(c: &mut Criterion) {
    let x = rand::random::<Block>();
    let aes = Aes::new(&x);
    let blk = rand::random::<Block>();
    let blks = rand::random::<[Block;8]>();

    c.bench_function("aes::new", move |bench| {
        bench.iter(|| {
            black_box(Aes::new(&x));
        });
    });

    c.bench_function("aes::encrypt_block", move |bench| {
        bench.iter(|| {
            black_box(aes.encrypt_block(&blk));
        });
    });

    c.bench_function("aes::encrypt_blocks", move |bench| {
        let key = rand::random::<Block>();
        let aes = Aes::new(&key);
        bench.iter(|| {
            black_box(aes.encrypt_blocks::<8>(&blks));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);