//! Implement aes128
use aes::{
    cipher::{generic_array::GenericArray, typenum::U16, BlockEncrypt, KeyInit},
    Aes128Enc,
};

use crate::Block;

///The AES 128 struct
pub struct Aes(Aes128Enc);

impl Aes {
    /// New a Aes instance with `key`
    pub fn new(key: &Block) -> Self {
        let _key: [u8; 16] = (*key).into();
        Self(Aes128Enc::new_from_slice(&_key).unwrap())
    }

    /// Encrypt a block
    pub fn encrypt_block(&self, blk: &Block) -> Block {
        let mut buf = GenericArray::from(*blk);
        self.0.encrypt_block(&mut buf);
        Block::try_from_slice(buf.as_slice()).unwrap()
    }

    /// Encrypt many blocks
    pub fn encrypt_blocks<const N: usize>(&self, blks: &[Block; N]) -> [Block; N] {
        let mut buf: Vec<GenericArray<u8, U16>> =
            blks.iter().map(|x| GenericArray::from(*x)).collect();
        self.0.encrypt_blocks(&mut buf);

        buf.iter()
            .map(|x| Block::from(*x))
            .collect::<Vec<Block>>()
            .try_into()
            .unwrap()
    }
}
