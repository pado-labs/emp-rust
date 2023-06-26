use aes::{
    cipher::{BlockEncrypt, KeyInit},
    Aes128Enc,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_tool::{
    aes::{Aes, AesEmp},
    block::Block,
};
use generic_array::GenericArray;

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

    c.bench_function("aes::encrypt_blocks::<8>", move |bench| {
        let key = rand::random::<Block>();
        let aes = Aes::new(key);
        let blks = rand::random::<[Block; 8]>();

        bench.iter(|| {
            black_box(aes.encrypt_blocks::<8>(blks));
        });
    });

    c.bench_function("original aes::encrypt_blocks::<8>", move |bench| {
        let key = rand::random::<[u8; 16]>();
        let aes = Aes128Enc::new_from_slice(&key).unwrap();
        let blks = rand::random::<[u8; 16]>();
        let blks = GenericArray::from(blks);
        let mut blks = [blks; 8];
        bench.iter(|| {
            black_box(aes.encrypt_blocks(&mut blks));
        });
    });

    c.bench_function("aes-emp::new", move |bench| {
        bench.iter(|| {
            black_box(AesEmp::new(x));
        });
    });

    c.bench_function("aes-emp::encrypt_block", move |bench| {
        let key = rand::random::<Block>();
        let blk = rand::random::<Block>();
        let aes = AesEmp::new(key);
        bench.iter(|| {
            black_box(aes.encrypt_block(blk));
        });
    });
    c.bench_function("aes-emp::encrypt_many_blocks", move |bench| {
        let key = rand::random::<Block>();
        let blks = rand::random::<[Block;8]>();
        let aes = AesEmp::new(key);
        bench.iter(|| {
            black_box(aes.encrypt_many_blocks::<8>(blks));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
