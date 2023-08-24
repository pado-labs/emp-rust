//! Implement LPN with local linear code
//! More especifically, a local linear code is a random boolean matrix with at most D non-zero values in each row.

use crate::{Block, Prp};
use rayon::{prelude::*, ThreadPoolBuilder};
/// A struct related to LPN.
/// The `seed` defines a sparse binary matrix `A` with at most `D` non-zero values in each row.
/// Given a vector `x` and `e`, compute `y = Ax + e`.
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

    // Compute 4 rows
    #[inline]
    fn compute_four_rows_non_indep(&self, y: &mut [Block], x: &[Block], pos: usize, prp: &Prp) {
        let mut cnt = 0u64;
        let index = [0; D].map(|_| {
            let i: u64 = cnt;
            cnt += 1;
            Block::from([pos as u64, i])
        });

        let mut index: [Block; D] = prp.permute_many_blocks(index);
        let index: &mut [u32] = bytemuck::cast_slice_mut::<_, u32>(&mut index);

        for (i, y) in y[pos..].iter_mut().enumerate().take(4) {
            for ind in index[i * D..(i + 1) * D].iter_mut() {
                *ind &= self.mask;
                *ind = if *ind >= self.k { *ind - self.k } else { *ind };

                *y ^= x[*ind as usize];
            }
        }
    }

    #[inline]
    fn compute_four_rows_indep(&self, y: &mut [Block], x: &[Block], pos: usize, prp: &Prp) {
        let mut cnt = 0u64;
        let index = [0; D].map(|_| {
            let i = cnt;
            cnt += 1;
            Block::from([pos as u64, i])
        });

        let mut index = prp.permute_many_blocks(index);
        let index = bytemuck::cast_slice_mut::<_, u32>(&mut index);

        for (i, y) in y.iter_mut().enumerate().take(4) {
            for ind in index[i * D..(i + 1) * D].iter_mut() {
                *ind &= self.mask;
                *ind = if *ind >= self.k { *ind - self.k } else { *ind };

                *y ^= x[*ind as usize];
            }
        }
    }

    // Compute one row.
    #[inline]
    fn compute_one_row(&self, y: &mut [Block], x: &[Block], pos: usize, prp: &Prp) {
        let block_size = (D + 4 - 1) / 4;
        let mut index = (0..block_size)
            .map(|i| Block::from([pos as u64, i as u64]))
            .collect::<Vec<Block>>();
        prp.permute_block_slice(&mut index);
        let index = bytemuck::cast_slice_mut::<_, u32>(&mut index);

        for ind in index.iter_mut().take(D) {
            *ind &= self.mask;
            *ind = if *ind >= self.k { *ind - self.k } else { *ind };
            y[pos] ^= x[*ind as usize];
        }
    }

    /// Compute Ax + e
    pub fn compute_naive(&self, y: &mut [Block], x: &[Block]) {
        assert_eq!(x.len() as u32, self.k);
        assert!(x.len() >= D);
        let prp = Prp::new(self.seed);
        let batch_size = y.len() / 4;

        for i in 0..batch_size {
            self.compute_four_rows_non_indep(y, x, i * 4, &prp);
        }

        for i in batch_size * 4..y.len() {
            self.compute_one_row(y, x, i, &prp);
        }
    }

    // Thread task.
    fn task(&self, y: &mut [Block], x: &[Block], start: usize, end: usize) {
        let prp = Prp::new(self.seed);
        y.par_chunks_exact_mut(4).enumerate().for_each(|(i, y)| {
            self.compute_four_rows_indep(y, x, i * 4, &prp);
        });

        let len = end - start;
        let size = len - len % 4;

        for i in size..len {
            self.compute_one_row(y, x, i, &prp);
        }
        // let mut pos = start;
        // while pos < end - 4 {
        //     self.compute_four_rows_non_indep(y, x, pos, &prp);
        //     pos += 4;
        // }

        // while pos < end {
        //     self.compute_one_row(y, x, pos, &prp);
        //     pos += 1;
        // }
    }

    /// Compute Ax+e with multiple threads.
    pub fn compute(&self, y: &mut [Block], x: &[Block]) {
        assert_eq!(x.len() as u32, self.k);
        assert!(x.len() >= D);
        let prp = Prp::new(self.seed);
        let size = y.len() - (y.len() % 4);

        y.par_chunks_exact_mut(4).enumerate().for_each(|(i, y)| {
            self.compute_four_rows_indep(y, x, i * 4, &prp);
        });

        for i in size..y.len() {
            self.compute_one_row(y, x, i, &prp);
        }
    }

    /// Compute Ax+e with customized threads.
    pub fn compute_with_customized_threads(&self, y: &mut [Block], x: &[Block], threads: usize) {
        assert_eq!(x.len() as u32, self.k);
        assert!(x.len() >= D);
        let prp = Prp::new(self.seed);
        let size = y.len() - (y.len() % 4);

        let pool = ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();

        pool.install(|| {
            y.par_chunks_exact_mut(4).enumerate().for_each(|(i, y)| {
                self.compute_four_rows_indep(y, x, i * 4, &prp);
            });

            for i in size..y.len() {
                self.compute_one_row(y, x, i, &prp);
            }
        });
    }

    ///
    pub fn compute_with(&self, y: &mut [Block], x: &[Block], threads: usize) {
        assert_eq!(x.len() as u32, self.k);
        assert!(x.len() >= D);
        //let prp = Prp::new(self.seed);

        let pool = ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();

        let width = y.len() / threads;
        let len = y.len();

        pool.install(|| {
            if width > 0 {
                y.par_chunks_exact_mut(width).for_each(|y| {
                    //let start = i * width;
                    //let end = std::cmp::min((i + 1) * width, len);

                    self.task(y, x, 0, width);
                });
            }
            // vec![0; threads].par_iter().for_each(|i| {
            //     let start = i * width;
            //     let end = std::cmp::min((i + 1) * width, y.len());
            //     self.task(y, x, start, end);
            // })

            let start = (threads - 1) * width;
            let end = len;
            self.task(y, x, start, end);
        });

        // for i in 0..threads {
        //     let start = i * width;
        //     let end = std::cmp::min((i + 1) * width, y.len());
        //     self.task(y, x, start, end);
        // }

        //let start = (threads - 1) * width;
        // let start = (threads - 1) * width;
        // let end = len;
        // self.task(y, x, start, end);
    }
}

#[test]
fn lpn_test() {
    use crate::prg::Prg;
    let k = 20;
    let n = 6;
    let lpn = Lpn::<10>::new(Block::ZERO, k);
    let mut x = vec![Block::ZERO; k as usize];
    let mut y = vec![Block::ZERO; n];
    let mut prg = Prg::new();
    prg.random_blocks(&mut x);
    prg.random_blocks(&mut y);
    let mut z = y.clone();
    let mut zz = y.clone();
    let mut zzz = y.clone();
    lpn.compute_with_customized_threads(&mut y, &x, 8);
    lpn.compute_naive(&mut z, &x);
    lpn.compute(&mut zz, &x);
    lpn.compute_with(&mut zzz, &x, 8);

    assert_eq!(y, z);
    assert_eq!(y, zz);
    assert_eq!(y, zzz);
}
