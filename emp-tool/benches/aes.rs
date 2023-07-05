use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_tool::{aes::Aes, block::Block};

fn criterion_benchmark(c: &mut Criterion) {
    let x = rand::random::<Block>();
    let aes = Aes::new(x);
    let blk = rand::random::<Block>();

    c.bench_function("aes::new", move |bench| {
        bench.iter(|| {
            let z = Aes::new(black_box(x));
            black_box(black_box(z));
        });
    });

    c.bench_function("aes::encrypt_block", move |bench| {
        bench.iter(|| {
            let z = aes.encrypt_block(black_box(blk));
            black_box(z);
        });
    });

    c.bench_function("aes::encrypt_many_blocks::<8>", move |bench| {
        let key = rand::random::<Block>();
        let aes = Aes::new(key);
        let blks = rand::random::<[Block; 8]>();

        bench.iter(|| {
            let z = aes.encrypt_many_blocks(black_box(blks));
            black_box(z);
        });
    });

    c.bench_function("aes::encrypt_block_slice::<8>", move |bench| {
        let key = rand::random::<Block>();
        let aes = Aes::new(key);
        let mut blks = rand::random::<[Block; 8]>();

        bench.iter(|| {
            let z = aes.encrypt_block_slice(black_box(&mut blks));
            black_box(z);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
