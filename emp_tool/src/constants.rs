//! Define constants used in the libraries.

use std::mem;

use crate::block::Block;

/// Network buffer size, default `1MB`.
pub const NETWORK_BUFFER_SIZE: usize = 1024 * 1024;

/// Party PUBLIC
pub const PUBLIC: usize = 0;

/// Party ALICE
pub const ALICE: usize = 1;

/// Party BOB
pub const BOB: usize = 2;

/// The constant block with value `0`.
pub const ZERO_BLOCK: Block = Block(unsafe { mem::transmute(0u128) });

/// The constant block with value `0xFFFF_FFFF_FFFF_FFFF`.
pub const ONES_BLOCK: Block = Block(unsafe { mem::transmute(u128::MAX) });

/// The select array with `ZERO_BLOCK` and `ONES_BLOCK`.
pub const SELECT_MASK: [Block; 2] = [ZERO_BLOCK, ONES_BLOCK];
