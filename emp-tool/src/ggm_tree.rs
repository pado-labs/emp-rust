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

    /// Generate tree.\
    /// Take as input a `seed`. \
    /// The generated tree is stored in `tree`. \
    /// Output two block slices used for OT.
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
                self.tkprp.expand_4to8(&mut buf, &mut tree[i..]);
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

    // The following implementation is slower due to cach missing problems.
    //     pub fn gen_new(&self, seed: Block, tree: &mut [Block], k0: &mut [Block], k1: &mut [Block]) {
    //         let mut buf = vec![ZERO_BLOCK; (1 << self.depth) - 1];
    //         buf[0] = seed;

    //         self.tkprp.expand_1to2(&mut buf[1..], seed);
    //         k0[0] = buf[1];
    //         k1[0] = buf[2];

    //         let (prev, curr) = buf.split_at_mut(3);
    //         self.tkprp.expand_2to4(&mut curr[0..4], &prev[1..3]);
    //         k0[1] = curr[0] ^ curr[2];
    //         k1[1] = curr[1] ^ curr[3];

    //         for h in 2..self.depth - 1 {
    //             k0[h] = ZERO_BLOCK;
    //             k1[h] = ZERO_BLOCK;
    //             let sz = 1 << h;
    //             let (prev, curr) = buf.split_at_mut(2 * sz - 1);
    //             for i in (0..sz).step_by(4) {
    //                 self.tkprp
    //                     .expand_4to8(&mut curr[2 * i..2 * i + 8], &prev[sz - 1 + i..sz + 3 + i]);
    //                 k0[h] ^= curr[2 * i];
    //                 k0[h] ^= curr[2 * i + 2];
    //                 k0[h] ^= curr[2 * i + 4];
    //                 k0[h] ^= curr[2 * i + 6];
    //                 k1[h] ^= curr[2 * i + 1];
    //                 k1[h] ^= curr[2 * i + 3];
    //                 k1[h] ^= curr[2 * i + 5];
    //                 k1[h] ^= curr[2 * i + 7];
    //             }
    //         }
    //         let exp = 1 << (self.depth - 1);
    //         tree.copy_from_slice(&buf[exp - 1..]);
    //     }
}

// #[test]
// fn ggm_test() {
//     let depth = 5;
//     let mut tree = vec![ZERO_BLOCK; 1 << (depth - 1)];
//     let mut k0 = vec![ZERO_BLOCK; depth - 1];
//     let mut k1 = vec![ZERO_BLOCK; depth - 1];

//     let ggm = GgmTree::new(depth);

//     ggm.gen(ZERO_BLOCK, &mut tree, &mut k0, &mut k1);
//     println!("2: {:?}", tree);
//     println!("2: {:?}", k0);
//     println!("2: {:?}", k1);
// }
