use criterion::{black_box, criterion_group, criterion_main, Criterion};

use emp_tool::block::Block;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = ChaCha12Rng::from_entropy();
    let a: [u8; 16] = rng.gen();
    let b: [u8; 16] = rng.gen();
    let a = Block::new(&a);
    let b = Block::new(&b);
    const SIZE: usize = 1000;
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..SIZE {
        x.push(Block::from(rng.gen::<u128>()));
        y.push(Block::from(rng.gen::<u128>()));
    }
    let t = x.clone();
    let f = y.clone();

    let exp = rand::random::<u128>();

    c.bench_function("Block::clmul", move |bench| {
        bench.iter(|| {
            black_box(black_box(a).clmul(black_box(&b)));
        });
    });

    c.bench_function("Block::xor", move |bench| {
        bench.iter(|| {
            black_box(a ^ b);
        });
    });

    c.bench_function("Block::or", move |bench| {
        bench.iter(|| {
            black_box(a | b);
        });
    });

    c.bench_function("Block::and", move |bench| {
        bench.iter(|| {
            black_box(a & b);
        });
    });

    c.bench_function("Block::default", move |bench| {
        bench.iter(|| {
            black_box(Block::default());
        });
    });

    c.bench_function("Block::equal", move |bench| {
        bench.iter(|| {
            black_box(a == b);
        });
    });

    c.bench_function("Block::gfmul", move |bench| {
        bench.iter(|| {
            black_box(a.gfmul(&b));
        });
    });

    c.bench_function("Block::reduce", move |bench| {
        bench.iter(|| {
            black_box(Block::reduce(&a, &b));
        });
    });

    c.bench_function("Block::mul", move |bench| {
        bench.iter(|| {
            black_box(a * b);
        });
    });

    c.bench_function("Block::get_lsb", move |bench| {
        bench.iter(|| {
            black_box(a.get_lsb());
        });
    });

    c.bench_function("Block::inn_prod_no_red", move |bench| {
        bench.iter(|| {
            black_box(Block::inn_prdt_no_red(&x, &y));
        });
    });

    c.bench_function("Block::inn_prod_red", move |bench| {
        bench.iter(|| {
            black_box(Block::inn_prdt_red(&t, &f));
        });
    });

    c.bench_function("Block::pow", move |bench| {
        bench.iter(|| {
            black_box(a.pow(exp));
        });
    });

    c.bench_function("Block::inverse", move |bench| {
        bench.iter(|| {
            black_box(a.inverse());
        });
    });

    c.bench_function("Block::sigma", move |bench| {
        bench.iter(|| {
            black_box(Block::sigma(a));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
