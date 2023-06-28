//! Define hashes based on AES.

use crate::{aes::Aes, Block, ZERO_BLOCK};

/// Correlation-robust hash function for 128-bit inputs
/// (cf. <https://eprint.iacr.org/2019/074>, §7.2).
/// The function computes `π(x) ⊕ x`.
/// π(x) = AES(key=0x0,x)
pub struct CrHash(Aes);

impl CrHash {
    /// New a function with zero key.
    #[inline(always)]
    pub fn new() -> Self {
        Self(Aes::new(ZERO_BLOCK))
    }

    /// New a function with key.
    #[inline(always)]
    pub fn new_with_key(key: Block) -> Self {
        Self(Aes::new(key))
    }

    /// Hash one block.
    #[inline(always)]
    pub fn hash_block(&self, blk: Block) -> Block {
        self.0.encrypt_block(blk) ^ blk
    }

    /// Hash many blocks.
    #[inline(always)]
    pub fn hash_many_blocks<const N: usize>(&self, blks: [Block; N]) -> [Block; N] {
        let mut res = self.0.encrypt_many_blocks::<N>(blks);
        for i in 0..N {
            res[i] ^= blks[i]
        }
        res
    }
}
/// Circular correlation-robust hash function
/// (cf.<https://eprint.iacr.org/2019/074>, §7.3).
///
/// The function computes `H(σ(x))`, where `H` is a correlation-robust hash
/// function and `σ(x₀ || x₁) = (x₀ ⊕ x₁) || x₁`.
pub struct CcrHash(Aes);
impl CcrHash {
    /// New a function with zero key.
    #[inline(always)]
    pub fn new() -> Self {
        Self(Aes::new(ZERO_BLOCK))
    }

    /// New a function with key.
    #[inline(always)]
    pub fn new_with_key(key: Block) -> Self {
        Self(Aes::new(key))
    }

    /// Hash one block.
    #[inline(always)]
    pub fn hash_block(&self, blk: Block) -> Block {
        let t = Block::sigma(blk);
        self.0.encrypt_block(t) ^ t
    }

    /// Hash many blocks.
    #[inline(always)]
    pub fn hash_many_blocks<const N: usize>(&self, blks: [Block; N]) -> [Block; N] {
        let mut t = [ZERO_BLOCK; N];
        for i in 0..N {
            t[i] = Block::sigma(blks[i]);
        }
        let mut res = self.0.encrypt_many_blocks::<N>(t);
        for i in 0..N {
            res[i] ^= blks[i]
        }
        res
    }
}
/// Tweakable circular correlation robust hash function
/// (cf.<https://eprint.iacr.org/2019/074>, §7.4).
///
/// The function computes `π(π(x) ⊕ i) ⊕ π(x)`.
pub struct TccrHash(Aes);
impl TccrHash {
    /// New a function with zero key.
    #[inline(always)]
    pub fn new() -> Self {
        Self(Aes::new(ZERO_BLOCK))
    }

    /// New a function with key.
    #[inline(always)]
    pub fn new_with_key(key: Block) -> Self {
        Self(Aes::new(key))
    }

    /// Hash one block.
    #[inline(always)]
    pub fn hash_block(&self, blk: Block, id: u64) -> Block {
        let y = self.0.encrypt_block(blk);
        let idb = Block::from([0u64, id]);
        self.0.encrypt_block(y ^ idb) ^ y
    }

    /// Hash many blocks.
    #[inline(always)]
    pub fn hash_many_blocks<const N: usize>(&self, blks: [Block; N], ids: [u64;N]) -> [Block; N] {
        let y = self.0.encrypt_many_blocks::<N>(blks);
        let mut idsb = ids.map(|x|Block::from([0u64,x]));
        for i in 0..N {
            idsb[i] ^= y[i];
        }
        let mut res = self.0.encrypt_many_blocks::<N>(idsb);
        for i in 0..N {
            res[i] ^= y[i]
        }
        res
    }
}
