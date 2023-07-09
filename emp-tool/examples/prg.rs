use std::time::Instant;

use emp_tool::{prg::Prg, ZERO_BLOCK};

fn prg_perf() {
    let mut length = 2usize;
    while length <= 8192 {
        let times = 1024 * 1024 * 32 / length;
        let start = Instant::now();
        let mut blks = vec![ZERO_BLOCK; length];
        let mut prg = Prg::new();
        for _ in 0..times {
            prg.random_blocks(&mut blks);
        }

        let interval = start.elapsed().as_micros() as f64;
        println!(
            "PRG speed with block size {}:\t {}\t Gbps",
            length,
            ((length * times * 128) as f64) / (interval + 0.0) / 1000.0
        );
        length *= 2;
    }
}

pub fn main() {
    prg_perf();
}
