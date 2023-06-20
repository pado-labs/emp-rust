use aes::{Aes128Enc, cipher::{KeyInit, BlockEncrypt}};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_tool::{aes::Aes, block::Block};
use generic_array::GenericArray;

fn criterion_benchmark(c: &mut Criterion) {
    let x = rand::random::<Block>();
    let aes = Aes::new(&x);
    let blk = rand::random::<Block>();
    let blks = rand::random::<[Block; 8]>();

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

    c.bench_function("aes::encrypt_blocks::<8>", move |bench| {
        let key = rand::random::<Block>();
        let aes = Aes::new(&key);
        bench.iter(|| {
            black_box(aes.encrypt_blocks::<8>(&blks));
        });
    });

    c.bench_function("original aes::encrypt_blocks::<8>", move |bench| {
        let key = rand::random::<[u8;16]>();
        let aes = Aes128Enc::new_from_slice(&key).unwrap();
        let blks = rand::random::<[u8; 16]>();
        let blks = GenericArray::from(blks);
        let mut blks = [blks; 8];
        bench.iter(|| {
            black_box(aes.encrypt_blocks(&mut blks));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
