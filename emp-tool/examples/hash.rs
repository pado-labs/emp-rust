use std::time::Instant;

use emp_tool::{hash::Hash, prg::Prg, Block};

fn hash_perf() {
    let mut length = 2usize;
    while length <= 8192 {
        let times = 1024 * 1024 * 32 / length;
        let mut data = vec![Block::ZERO; length];
        let mut prg = Prg::new();
        prg.random_blocks(&mut data);

        let mut hasher = Hash::new();
        let start = Instant::now();
        for _ in 0..times {
            hasher.update_block_slice(&data);
            hasher.finalize();
        }

        let interval = start.elapsed().as_micros() as f64;
        println!(
            "Hash speed with block size {}:\t {}\t Gbps",
            length,
            ((length * times * 128) as f64) / (interval + 0.0) / 1000.0
        );
        length *= 2;
    }
}

pub fn main() {
    hash_perf();
}
