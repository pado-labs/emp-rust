//! Define constants used in the libraries.

use std::mem;

use crate::block::Block;

/// Network buffer size, default 1MB.
pub const NETWORK_BUFFER_SIZE: usize = 1024 * 1024;

/// Party PUBLIC
pub const PUBLIC: usize = 0;

/// Party ALICE
pub const ALICE: usize = 1;

/// Party BOB
pub const BOB: usize = 2;

pub const ZERO_BLOCK: Block = Block(unsafe { mem::transmute(0u128) });
pub const ONES_BLOCK: Block = Block(unsafe { mem::transmute(u128::MAX) });
pub const SELECT_MASK: [Block; 2] = [ZERO_BLOCK, ONES_BLOCK];
