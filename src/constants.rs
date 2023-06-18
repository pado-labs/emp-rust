//! Define constants used in the libraries.

/// Network buffer size, default 1MB.
pub const NETWORK_BUFFER_SIZE: usize = 1024 * 1024;

// /// Party PUBLIC
// pub const PUBLIC: usize = 0;

// /// Party ALICE
// pub const ALICE: usize = 1;

// /// Party BOB
// pub const BOB: usize = 2;

#[derive(Clone, Copy, PartialEq)]
pub enum PARTY {
    PUBLIC(usize),
    ALICE(usize),
    BOB(usize),
}

pub const PUBLIC: PARTY = PARTY::PUBLIC(0);
pub const ALICE: PARTY = PARTY::ALICE(1);
pub const BOB: PARTY = PARTY::BOB(2);
