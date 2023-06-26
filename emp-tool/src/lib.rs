#![deny(missing_docs)]

//! This crate defines and implements basic tools for MPC
#![cfg(target_arch = "aarch64")]
#![feature(stdsimd)]
pub mod aes;

pub mod block;
pub mod constants;
pub mod io_channel;
pub mod utils;
pub mod sse2neon;

pub use block::Block;
pub use constants::{ALICE, BOB, NETWORK_BUFFER_SIZE, ONES_BLOCK, SELECT_MASK, ZERO_BLOCK};
pub use io_channel::{CommandLineOpt, IOChannel, NetIO};
pub use utils::{pack_bits_to_bytes, unpack_bytes_to_bits};
pub use sse2neon::*;