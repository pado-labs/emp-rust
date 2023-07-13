#![deny(missing_docs)]

//! This crate defines and implements basic tools for MPC
#![cfg_attr(target_arch = "aarch64", feature(stdsimd))]
pub mod aes;

pub mod block;
pub mod constants;
pub mod ggm_tree;
pub mod hash;
pub mod io_channel;
pub mod prg;
pub mod sse2neon;
pub mod tkprp;
pub mod utils;

pub use block::Block;
pub use constants::{ALICE, BOB, NETWORK_BUFFER_SIZE, ONES_BLOCK, SELECT_MASK, ZERO_BLOCK};
pub use ggm_tree::GgmTree;
pub use hash::{CcrHash, CrHash, TccrHash};
pub use io_channel::{CommandLineOpt, IOChannel, NetIO};
pub use sse2neon::*;
pub use tkprp::TwoKeyPrp;
pub use utils::{pack_bits_to_bytes, unpack_bytes_to_bits};
