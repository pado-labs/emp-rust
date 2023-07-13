//! Implement the two-key PRG as G(k) = PRF_seed0(k)\xor k || PRF_seed1(k)\xor k

use crate::{aes::Aes, Block, ZERO_BLOCK};

/// Struct of two-key prp.
pub struct TwoKeyPrp([Aes; 2]);

impl TwoKeyPrp {
    /// New an instance of TwoKeyPrp
    #[inline(always)]
    pub fn new(seeds: [Block; 2]) -> Self {
        Self([Aes::new(seeds[0]), Aes::new(seeds[1])])
    }

    /// expand 1 to 2
    #[inline(always)]
    pub fn expand_1to2(&self, children: &mut [Block], parent: Block) {
        children[0] = parent;
        children[1] = parent;
        Aes::para_encrypt::<2, 1>(self.0, children);
        children[0] ^= parent;
        children[1] ^= parent;
    }

    /// expand 1 to 2
    #[inline(always)]
    pub fn node_expand_1to2(&self, parent: Block) -> [Block; 2] {
        let mut children = [ZERO_BLOCK; 2];
        children[0] = parent;
        children[1] = parent;
        Aes::para_encrypt::<2, 1>(self.0, &mut children);
        children[0] ^= parent;
        children[1] ^= parent;

        children
    }

    /// expand 2 to 4
    #[inline(always)]
    pub fn expand_2to4_inplace(&self, children: &mut [Block]) {
        let mut tmp = [ZERO_BLOCK; 4];
        tmp[3] = children[1];
        tmp[1] = children[1];
        tmp[2] = children[0];
        tmp[0] = children[0];

        Aes::para_encrypt::<2, 2>(self.0, &mut tmp);

        children[3] = children[1] ^ tmp[3];
        children[2] = children[1] ^ tmp[1];
        children[1] = children[0] ^ tmp[2];
        children[0] = children[0] ^ tmp[0];
    }

    /// expand 2 to 4
    #[inline(always)]
    pub fn expand_2to4(&self, children: &mut [Block], parent: &[Block]) {
        let mut tmp = [ZERO_BLOCK; 4];
        children[3] = parent[1];
        children[2] = parent[1];
        children[1] = parent[0];
        children[0] = parent[0];

        tmp[3] = parent[1];
        tmp[1] = parent[1];
        tmp[2] = parent[0];
        tmp[0] = parent[0];

        Aes::para_encrypt::<2, 2>(self.0, &mut tmp);

        children[3] ^= tmp[3];
        children[2] ^= tmp[1];
        children[1] ^= tmp[2];
        children[0] ^= tmp[0];
    }
    /// expand 2 to 4
    //     p[0]            p[1]
    // c[0]    c[1]    c[2]    c[3]
    // t[0]    t[2]    t[1]    t[3]
    #[inline(always)]
    pub fn node_expand_2to4(&self, parent: [Block; 2]) -> [Block; 4] {
        let mut tmp = [ZERO_BLOCK; 4];
        let mut children = [ZERO_BLOCK; 4];

        children[3] = parent[1];
        children[2] = parent[1];
        children[1] = parent[0];
        children[0] = parent[0];

        tmp[3] = parent[1];
        tmp[1] = parent[1];
        tmp[2] = parent[0];
        tmp[0] = parent[0];

        Aes::para_encrypt::<2, 2>(self.0, &mut tmp);

        children[3] ^= tmp[3];
        children[2] ^= tmp[1];
        children[1] ^= tmp[2];
        children[0] ^= tmp[0];

        children
    }

    /// expand 4 to 8
    #[inline(always)]
    pub fn expand_4to8_inplace(&self, children: &mut [Block]) {
        let mut tmp = [ZERO_BLOCK; 8];
        tmp[7] = children[3];
        tmp[3] = children[3];
        tmp[6] = children[2];
        tmp[2] = children[2];
        tmp[5] = children[1];
        tmp[1] = children[1];
        tmp[4] = children[0];
        tmp[0] = children[0];

        Aes::para_encrypt::<2, 4>(self.0, &mut tmp);

        children[7] = children[3] ^ tmp[7];
        children[6] = children[3] ^ tmp[3];
        children[5] = children[2] ^ tmp[6];
        children[4] = children[2] ^ tmp[2];
        children[3] = children[1] ^ tmp[5];
        children[2] = children[1] ^ tmp[1];
        children[1] = children[0] ^ tmp[4];
        children[0] = children[0] ^ tmp[0];
    }
    /// expand 4 to 8
    #[inline(always)]
    pub fn expand_4to8(&self, children: &mut [Block], parent: &[Block]) {
        let mut tmp = [ZERO_BLOCK; 8];
        children[7] = parent[3];
        children[6] = parent[3];
        children[5] = parent[2];
        children[4] = parent[2];
        children[3] = parent[1];
        children[2] = parent[1];
        children[1] = parent[0];
        children[0] = parent[0];

        tmp[7] = parent[3];
        tmp[3] = parent[3];
        tmp[6] = parent[2];
        tmp[2] = parent[2];
        tmp[5] = parent[1];
        tmp[1] = parent[1];
        tmp[4] = parent[0];
        tmp[0] = parent[0];

        Aes::para_encrypt::<2, 4>(self.0, &mut tmp);

        children[7] ^= tmp[7];
        children[6] ^= tmp[3];
        children[5] ^= tmp[6];
        children[4] ^= tmp[2];
        children[3] ^= tmp[5];
        children[2] ^= tmp[1];
        children[1] ^= tmp[4];
        children[0] ^= tmp[0];
    }
    /// expand 4 to 8
    //     p[0]            p[1]            p[2]            p[3]
    // c[0]    c[1]    c[2]    c[3]    c[4]    c[5]    c[6]    c[7]
    // t[0]    t[4]    t[1]    t[5]    t[2]    t[6]    t[3]    t[7]
    #[inline(always)]
    pub fn node_expand_4to8(&self, parent: [Block; 4]) -> [Block; 8] {
        let mut tmp = [ZERO_BLOCK; 8];
        let mut children = [ZERO_BLOCK; 8];

        children[7] = parent[3];
        children[6] = parent[3];
        children[5] = parent[2];
        children[4] = parent[2];
        children[3] = parent[1];
        children[2] = parent[1];
        children[1] = parent[0];
        children[0] = parent[0];

        tmp[7] = parent[3];
        tmp[3] = parent[3];
        tmp[6] = parent[2];
        tmp[2] = parent[2];
        tmp[5] = parent[1];
        tmp[1] = parent[1];
        tmp[4] = parent[0];
        tmp[0] = parent[0];

        Aes::para_encrypt::<2, 4>(self.0, &mut tmp);

        children[7] ^= tmp[7];
        children[6] ^= tmp[3];
        children[5] ^= tmp[6];
        children[4] ^= tmp[2];
        children[3] ^= tmp[5];
        children[2] ^= tmp[1];
        children[1] ^= tmp[4];
        children[0] ^= tmp[0];

        children
    }
}
