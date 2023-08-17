use std::time::Instant;

use emp_tool::{
    block::Block,
    constants::{ALICE, BOB},
    io_channel::{CommandLineOpt, IOChannel, NetIO},
};
use structopt::StructOpt;

fn netio_perf(io: &mut NetIO, party: usize) {
    if party == ALICE {
        let mut length = 2usize;
        while length <= 8192 * 16 {
            let times = 1024 * 1024 * 128 / length;
            let start = Instant::now();
            let blks = vec![Block::default(); length];
            for _ in 0..times {
                io.send_block_vec(&blks).unwrap();
            }

            let interval = start.elapsed().as_micros() as f64;
            println!(
                "Loopback speed with block size {}:\t {}\t Gbps",
                length,
                ((length * times * 128) as f64) / (interval + 0.0) / 1000.0
            );
            length *= 2;
        }
    } else if party == BOB {
        let mut length = 2usize;
        while length <= 8192 * 16 {
            let times = 1024 * 1024 * 128 / length;
            for _ in 0..times {
                let _blk = io.recv_block_vec(length).unwrap();
            }
            length *= 2;
        }
    }
}

// Run the main function in two terminals
// cargo run --release --example netio -- --party 1
// cargo run --release --example netio -- --party 2
pub fn main() {
    let opt = CommandLineOpt::from_args();
    let party = opt.party;
    let is_server = if party == ALICE { true } else { false };
    let mut io = NetIO::new(is_server, "127.0.0.1:12345").unwrap();
    netio_perf(&mut io, party);
}
