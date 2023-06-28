use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_tool::{aes::Aes, block::Block};

fn criterion_benchmark(c: &mut Criterion) {
    let x = rand::random::<Block>();
    let aes = Aes::new(x);
    let blk = rand::random::<Block>();

    c.bench_function("aes::new", move |bench| {
        bench.iter(|| {
            black_box(Aes::new(x));
        });
    });

    c.bench_function("aes::encrypt_block", move |bench| {
        bench.iter(|| {
            black_box(aes.encrypt_block(blk));
        });
    });

    c.bench_function("aes::encrypt_many_blocks::<8>", move |bench| {
        let key = rand::random::<Block>();
        let aes = Aes::new(key);
        let blks = rand::random::<[Block; 8]>();

        bench.iter(|| {
            black_box(aes.encrypt_many_blocks::<8>(blks));
        });
    });

    c.bench_function("aes::encrypt_vec_blocks::<8>", move |bench| {
        let key = rand::random::<Block>();
        let aes = Aes::new(key);
        let blks = rand::random::<[Block; 8]>();

        bench.iter(|| {
            black_box(aes.encrypt_vec_blocks(&blks));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
