#![deny(missing_docs)]

//! This crate defines and implements basic tools for MPC
#![cfg_attr(target_arch = "aarch64", feature(stdsimd))]
pub mod aes;

pub mod block;
pub mod constants;
pub mod ggm_tree;
pub mod hash;
pub mod io_channel;
pub mod lpn;
pub mod prg;
pub mod prp;
pub mod sse2neon;
pub mod tkprp;
pub mod utils;

pub use aes::Aes;
pub use block::Block;
pub use constants::{ALICE, BOB, PUBLIC};
pub use hash::{CcrHash, CrHash, TccrHash};
pub use io_channel::{CommandLineOpt, IOChannel, NetIO};
pub use utils::{pack_bits_to_bytes, unpack_bytes_to_bits};
