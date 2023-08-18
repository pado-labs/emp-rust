//! Implement LPN with local linear code
//! More especifically, a local linear code is a random boolean matrix with exact D non-zero values in each row.

use rand_core::SeedableRng;

use crate::{prg::Prg, Block};

/// A struct related to LPN.
/// The `seed` defines a sparse binary matrix A with exact `D` non-zero values in each row.
/// Given a vector `x` and `e`, compute y = Ax + e.
pub struct Lpn<const D: usize> {
    // The seed to generate the random sparse matrix A.
    seed: Block,

    // The length of the secret, i.e., x.
    k: u32,

    // A mask to optimize reduction operation.
    mask: u32,
}

impl<const D: usize> Lpn<D> {
    /// New an LPN instance
    pub fn new(seed: Block, k: u32) -> Self {
        let mut mask = 1;
        while mask < k {
            mask <<= 1;
            mask |= 0x1;
        }
        Self { seed, k, mask }
    }

    // Compute N rows
    fn compute_rows<const N: usize>(
        &self,
        y: &mut [Block],
        x: &[Block],
        pos: usize,
        prg: &mut Prg,
    ) {
        // assert_eq!(x.len() as u32, self.k);
        // assert!(x.len() >= D);

        // let mut cnt = 0u64;
        // let buf = [0; D].map(|_| {
        //     let x = cnt;
        //     cnt += 1;
        //     Block::from([pos as u64, x as u64])
        // });

        // let buf = prp.permute_many_blocks(buf);
        let mut index = [0u32; D];
        prg.random_bytes(bytemuck::cast_slice_mut(&mut index));

        for i in 0..N {
            for j in 0..D {
                // index[j] &= self.mask;
                // index[j] = if index[j] >= self.k {
                //     index[j] - self.k
                // } else {
                //     index[j]
                // };

                // y[pos + i] ^= x[index[j] as usize];
                y[pos + i] ^= x[(index[j] % self.k) as usize];
            }
        }
    }

    // Compute one row.
    fn compute_one_row(&self, y: &mut [Block], x: &[Block], pos: usize, prg: &mut Prg) {
        let mut index = [0u32; D];
        prg.random_bytes(bytemuck::cast_slice_mut(&mut index));

        for j in 0..D {
            //     index[j] = if index[j] > self.k {
            //         index[j] - self.k
            //     } else {
            //         index[j]
            //     };

            y[pos] ^= x[(index[j] % self.k) as usize];
        }
    }

    /// Compute Ax + e
    pub fn compute_naive<const N: usize>(&self, y: &mut [Block], x: &[Block]) {
        assert_eq!(x.len() as u32, self.k);
        assert!(x.len() >= D);
        let mut prg = Prg::from_seed(self.seed);

        let batch_size = y.len() / N;

        for i in 0..batch_size {
            self.compute_rows::<N>(y, x, i * N, &mut prg);
        }

        for i in batch_size * N..y.len() {
            self.compute_one_row(y, x, i, &mut prg)
        }
    }
}
