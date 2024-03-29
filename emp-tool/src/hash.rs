//! Define hashes based on AES.

use sha2::{Digest, Sha256};

use crate::{aes::Aes, Block};

/// Correlation-robust hash function for 128-bit inputs
/// (cf. <https://eprint.iacr.org/2019/074>, §7.2).
/// The function computes `π(x) xor x`.
/// π(x) = AES(key=0x0,x)
pub struct CrHash(Aes);

impl CrHash {
    /// New a function with zero key.
    #[inline(always)]
    pub fn new() -> Self {
        Self(Aes::new(Block::ZERO))
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

impl Default for CrHash {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Circular correlation-robust hash function
/// (cf.<https://eprint.iacr.org/2019/074>, §7.3).
///
/// The function computes `H(sigma(x))`, where `H` is a correlation-robust hash
/// function and `sigma( x = x0 || x1 ) = x1 || (x0 xor x1)`.
/// `x0` and `x1` are the lower and higher halves of `x`, respectively.
pub struct CcrHash(Aes);
impl CcrHash {
    /// New a function with zero key.
    #[inline(always)]
    pub fn new() -> Self {
        Self(Aes::new(Block::ZERO))
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
        let mut t = [Block::ZERO; N];
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

impl Default for CcrHash {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
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
        Self(Aes::new(Block::ZERO))
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
        let idb = Block::from([id, 0u64]);
        self.0.encrypt_block(y ^ idb) ^ y
    }

    /// Hash many blocks.
    #[inline(always)]
    pub fn hash_many_blocks<const N: usize>(&self, blks: [Block; N], ids: [u64; N]) -> [Block; N] {
        let y = self.0.encrypt_many_blocks::<N>(blks);
        let mut idsb = ids.map(|x| Block::from([x, 0u64]));
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

impl Default for TccrHash {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// A wrapper of SHA256
pub struct Hash(Sha256);

impl Hash {
    /// Hash Digest Size
    pub const DIGEST_SIZE: usize = 32;

    /// New a hash instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self(Sha256::new())
    }

    /// Update bytes.
    #[inline(always)]
    pub fn update(&mut self, m: &[u8]) {
        self.0.update(m);
    }

    /// Finalize output
    #[inline(always)]
    pub fn finalize(&mut self) -> [u8; Hash::DIGEST_SIZE] {
        let hasher = self.0.clone();
        let mut res = [0u8; Hash::DIGEST_SIZE];
        res.copy_from_slice(&hasher.finalize());
        res
    }

    /// Update block.
    #[inline(always)]
    pub fn update_block(&mut self, m: Block) {
        self.update(m.as_ref());
    }

    /// Update block slice.
    #[inline(always)]
    pub fn update_block_slice(&mut self, m: &[Block]) {
        self.update(bytemuck::cast_slice(m));
    }

    /// Hash bytes once.
    #[inline(always)]
    pub fn hash_bytes_once(&mut self, m: &[u8]) -> [u8; Hash::DIGEST_SIZE] {
        self.update(m);
        self.finalize()
    }

    /// Hash blocks once.
    #[inline(always)]
    pub fn hash_blocks_once(&mut self, m: &[Block]) -> [u8; Hash::DIGEST_SIZE] {
        self.update_block_slice(m);
        self.finalize()
    }
}

impl Default for Hash {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn hash_test() {
    let h = CrHash::new();
    assert_eq!(
        h.hash_block(Block::ONES),
        Block::from(0xb19972c12db88c05f5a57a153673a4c0)
    );

    let h = CcrHash::new();
    assert_eq!(
        h.hash_block(Block::ONES),
        Block::from(0x9e10c525db2c0ea50a1fa067183cf807)
    );

    let h = TccrHash::new();
    assert_eq!(
        h.hash_block(Block::ONES, 1),
        Block::from(0x68e0f8bae7d74f1581fc3d4b682d6260)
    );
}
