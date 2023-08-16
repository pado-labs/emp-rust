//! Implement GGM tree for OT.

use crate::{Block, TwoKeyPrp, ZERO_BLOCK};

/// Struct of GGM
pub struct GgmTree {
    tkprp: TwoKeyPrp,
    depth: usize,
}

impl GgmTree {
    ///New GgmTree instance.
    #[inline(always)]
    pub fn new(depth: usize) -> Self {
        let tkprp = TwoKeyPrp::new([ZERO_BLOCK, Block::from(1u128)]);
        Self { tkprp, depth }
    }

    /// Input: `seed`: a seed.
    /// Output: `tree`: a GGM (binary tree) `tree`, with size `2^{depth-1}`
    /// Output: `k0`: XORs of all the left-node values in each level, with size `depth-1`.
    /// Output: `k1`: XORs of all the right-node values in each level, with size `depth-1`.
    /// This implementation is adopted from EMP Toolkit.
    pub fn gen(&self, seed: Block, tree: &mut [Block], k0: &mut [Block], k1: &mut [Block]) {
        let mut buf = vec![ZERO_BLOCK; 8];
        self.tkprp.expand_1to2(tree, seed);
        k0[0] = tree[0];
        k1[0] = tree[1];

        self.tkprp.expand_2to4(&mut buf, tree);
        k0[1] = buf[0] ^ buf[2];
        k1[1] = buf[1] ^ buf[3];
        tree[0..4].copy_from_slice(&buf[0..4]);

        for h in 2..self.depth - 1 {
            k0[h] = ZERO_BLOCK;
            k1[h] = ZERO_BLOCK;
            let sz = 1 << h;
            for i in (0..=sz - 4).rev().step_by(4) {
                self.tkprp.expand_4to8(&mut buf, &tree[i..]);
                k0[h] ^= buf[0];
                k0[h] ^= buf[2];
                k0[h] ^= buf[4];
                k0[h] ^= buf[6];
                k1[h] ^= buf[1];
                k1[h] ^= buf[3];
                k1[h] ^= buf[5];
                k1[h] ^= buf[7];

                tree[2 * i..2 * i + 8].copy_from_slice(&buf);
            }
        }
    }
}

#[test]
fn ggm_test() {
    let depth = 3;
    let mut tree = vec![ZERO_BLOCK; 1 << (depth - 1)];
    let mut k0 = vec![ZERO_BLOCK; depth - 1];
    let mut k1 = vec![ZERO_BLOCK; depth - 1];

    let ggm = GgmTree::new(depth);

    ggm.gen(ZERO_BLOCK, &mut tree, &mut k0, &mut k1);

    assert_eq!(
        tree,
        [
            Block::from(0x92A6DDEAA3E99F9BECB268BD9EF67C91),
            Block::from(0x9E7E9C02ED1E62385EE8A9EDDC63A2B5),
            Block::from(0xBD4B85E90AACBD106694537DB6251264),
            Block::from(0x230485DC4360014833E07D8D914411A2),
        ]
    );

    assert_eq!(
        k0,
        [
            Block::from(0x2E2B34CA59FA4C883B2C8AEFD44BE966),
            Block::from(0x2FED5803A945228B8A263BC028D36EF5),
        ]
    );

    assert_eq!(
        k1,
        [
            Block::from(0x7E46C568D1CD4972BB1A61F95DD80EDC),
            Block::from(0xBD7A19DEAE7E63706D08D4604D27B317),
        ]
    );
}
