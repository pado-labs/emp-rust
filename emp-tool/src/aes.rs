//! Implement aes128
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use crate::sse2neon::AES_SBOX;

#[cfg(target_arch = "aarch64")]
use crate::{
    _mm_aeskeygenassist_si128, _mm_castps_si128, _mm_castsi128_ps, _mm_cvtsi128_si32,
    _mm_shuffle_epi32, _mm_shuffle_ps, _mm_xor_si128,
};

use crate::Block;
///The AES 128 struct
#[derive(Copy, Clone, Debug)]
pub struct Aes([Block; 11]);

#[allow(unused_macros)]
macro_rules! expand_assist_x86 {
    ($v1:ident,$v2:ident,$v3:ident,$v4:ident,$sc:expr,$ac:expr) => {
        $v2 = _mm_aeskeygenassist_si128($v4, $ac);
        $v3 = _mm_castps_si128(_mm_shuffle_ps(
            _mm_castsi128_ps($v3),
            _mm_castsi128_ps($v1),
            16,
        ));
        $v1 = _mm_xor_si128($v1, $v3);
        $v3 = _mm_castps_si128(_mm_shuffle_ps(
            _mm_castsi128_ps($v3),
            _mm_castsi128_ps($v1),
            140,
        ));
        $v1 = _mm_xor_si128($v1, $v3);
        $v2 = _mm_shuffle_epi32($v2, $sc);
        $v1 = _mm_xor_si128($v1, $v2);
    };
}

#[allow(unused_macros)]
macro_rules! expand_assist_arm {
    ($v1:expr,$v2:expr,$v3:expr,$v4:expr, $sc:expr,$ac:expr) => {
        $v2 = _mm_aeskeygenassist_si128!($v4, $ac);
        $v3 = _mm_castps_si128!(_mm_shuffle_ps!(
            _mm_castsi128_ps!($v3),
            _mm_castsi128_ps!($v1),
            16
        ));
        $v1 = _mm_xor_si128!($v1, $v3);
        $v3 = _mm_castps_si128!(_mm_shuffle_ps!(
            _mm_castsi128_ps!($v3),
            _mm_castsi128_ps!($v1),
            140
        ));
        $v1 = _mm_xor_si128!($v1, $v3);
        $v2 = _mm_shuffle_epi32!($v2, $sc);
        $v1 = _mm_xor_si128!($v1, $v2);
    };
}

impl Aes {
    /// The AES_BLOCK_SIZE.
    pub const AES_BLOCK_SIZE: usize = 8;

    /// New an AES instance
    #[inline(always)]
    pub fn new(key: Block) -> Self {
        unsafe { Aes::aes_init(key) }
    }

    #[inline]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "aes")]
    unsafe fn aes_init(key: Block) -> Self {
        let mut kp = [Block::default(); 11];
        kp[0] = key;
        let mut x0 = key.0;
        let mut _x1 = _mm_setzero_si128();
        let mut x2 = _mm_setzero_si128();

        expand_assist_x86!(x0, _x1, x2, x0, 255, 1);
        kp[1] = Block(x0);

        expand_assist_x86!(x0, _x1, x2, x0, 255, 2);
        kp[2] = Block(x0);

        expand_assist_x86!(x0, _x1, x2, x0, 255, 4);
        kp[3] = Block(x0);

        expand_assist_x86!(x0, _x1, x2, x0, 255, 8);
        kp[4] = Block(x0);

        expand_assist_x86!(x0, _x1, x2, x0, 255, 16);
        kp[5] = Block(x0);

        expand_assist_x86!(x0, _x1, x2, x0, 255, 32);
        kp[6] = Block(x0);

        expand_assist_x86!(x0, _x1, x2, x0, 255, 64);
        kp[7] = Block(x0);

        expand_assist_x86!(x0, _x1, x2, x0, 255, 128);
        kp[8] = Block(x0);

        expand_assist_x86!(x0, _x1, x2, x0, 255, 27);
        kp[9] = Block(x0);

        expand_assist_x86!(x0, _x1, x2, x0, 255, 54);
        kp[10] = Block(x0);
        Self(kp)
    }

    #[inline]
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "aes")]
    unsafe fn aes_init(key: Block) -> Self {
        let mut kp = [Block::default(); 11];
        kp[0] = key;
        let mut x0 = key.0;
        let mut _x1 = vdupq_n_u8(0);
        let mut x2 = vdupq_n_u8(0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 1);
        kp[1] = Block(x0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 2);
        kp[2] = Block(x0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 4);
        kp[3] = Block(x0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 8);
        kp[4] = Block(x0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 16);
        kp[5] = Block(x0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 32);
        kp[6] = Block(x0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 64);
        kp[7] = Block(x0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 128);
        kp[8] = Block(x0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 27);
        kp[9] = Block(x0);

        expand_assist_arm!(x0, _x1, x2, x0, 255, 54);
        kp[10] = Block(x0);
        Self(kp)
    }

    /// Encrypt one block.
    #[inline(always)]
    pub fn encrypt_block(&self, blk: Block) -> Block {
        unsafe { self.encrypt_backend(blk) }
    }

    #[inline]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "aes")]
    unsafe fn encrypt_backend(&self, blk: Block) -> Block {
        let mut ctxt = _mm_xor_si128(blk.0, self.0[0].0);

        for key in self.0[1..10].iter() {
            ctxt = _mm_aesenc_si128(ctxt, key.0);
        }

        ctxt = _mm_aesenclast_si128(ctxt, self.0[10].0);
        Block(ctxt)
    }

    #[inline]
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "aes")]
    unsafe fn encrypt_backend(&self, blk: Block) -> Block {
        let mut ctxt = blk.0;

        for key in self.0.iter().take(9) {
            ctxt = vaesmcq_u8(vaeseq_u8(ctxt, key.0));
        }

        ctxt = veorq_u8(vaeseq_u8(ctxt, self.0[9].0), self.0[10].0);
        Block(ctxt)
    }

    /// Encrypt many blocks
    #[inline(always)]
    pub fn encrypt_many_blocks<const N: usize>(&self, blks: [Block; N]) -> [Block; N] {
        unsafe { self.unsafe_encrypt_many_blocks::<N>(blks) }
    }

    #[inline]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "aes")]
    unsafe fn unsafe_encrypt_many_blocks<const N: usize>(&self, blks: [Block; N]) -> [Block; N] {
        let mut ctxt = blks.map(|x| x.0);
        for ct in ctxt.iter_mut() {
            *ct = _mm_xor_si128(*ct, self.0[0].0);
        }

        for key in self.0[1..10].iter() {
            for ct in ctxt.iter_mut() {
                *ct = _mm_aesenc_si128(*ct, key.0);
            }
        }

        for ct in ctxt.iter_mut() {
            *ct = _mm_aesenclast_si128(*ct, self.0[10].0);
        }

        ctxt.map(|x| Block(x))
    }

    #[inline]
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "aes")]
    unsafe fn unsafe_encrypt_many_blocks<const N: usize>(&self, blks: [Block; N]) -> [Block; N] {
        let mut ctxt = blks.map(|x| x.0);

        for key in self.0.iter().take(9) {
            for ct in ctxt.iter_mut() {
                *ct = vaesmcq_u8(vaeseq_u8(*ct, key.0));
            }
        }

        for ct in ctxt.iter_mut() {
            *ct = veorq_u8(vaeseq_u8(*ct, self.0[9].0), self.0[10].0);
        }

        ctxt.map(Block)
    }

    /// Encrypt block slice
    #[inline(always)]
    pub fn encrypt_block_slice(&self, blks: &mut [Block]) {
        let len = blks.len();
        let mut buf = [Block::ZERO; Aes::AES_BLOCK_SIZE];
        for i in 0..len / Aes::AES_BLOCK_SIZE {
            buf.copy_from_slice(&blks[i * Aes::AES_BLOCK_SIZE..(i + 1) * Aes::AES_BLOCK_SIZE]);
            blks[i * Aes::AES_BLOCK_SIZE..(i + 1) * Aes::AES_BLOCK_SIZE]
                .copy_from_slice(&self.encrypt_many_blocks(buf));
        }

        let remain = len % Aes::AES_BLOCK_SIZE;
        if remain > 0 {
            macro_rules! encrypt_some {
                ($n:expr) => {{
                    if remain == $n {
                        let mut buf = [Block::ZERO; $n];
                        buf.copy_from_slice(&blks[len - remain..]);
                        blks[len - remain..].copy_from_slice(&self.encrypt_many_blocks(buf));
                    }
                }};
            }
            encrypt_some!(1);
            encrypt_some!(2);
            encrypt_some!(3);
            encrypt_some!(4);
            encrypt_some!(5);
            encrypt_some!(6);
            encrypt_some!(7);
        }
    }

    /// Encrypt many blocks with many keys.
    /// Input: `NK` AES keys `keys`, and `NK * NM` blocks `blks`
    /// Output: each batch of NM blocks encrypted by a corresponding AES key.
    /// Only handle the first `NK * NM` blocks of blks, do not handle the rest.
    #[inline(always)]
    pub fn para_encrypt<const NK: usize, const NM: usize>(keys: [Self; NK], blks: &mut [Block]) {
        assert!(blks.len() >= NM * NK);
        let mut ctxt = [Block::default(); NM];
        keys.iter().enumerate().for_each(|(i, key)| {
            ctxt.copy_from_slice(&blks[i * NM..(i + 1) * NM]);
            blks[i * NM..(i + 1) * NM].copy_from_slice(&key.encrypt_many_blocks(ctxt))
        });
    }
}

#[test]
fn aes_test() {
    let aes = Aes::new(Block::default());
    let c = aes.encrypt_block(Block::default());
    let res = Block::from(0x2e2b34ca59fa4c883b2c8aefd44be966);
    assert_eq!(c, res);

    macro_rules! encrypt_test {
        ($n:expr) => {{
            let blks = [Block::default(); $n];

            let d = aes.encrypt_many_blocks(blks);
            assert_eq!(d, [res; $n]);

            let mut f = [Block::default(); $n];
            aes.encrypt_block_slice(&mut f);
            assert_eq!(f, [res; $n]);
        }};
    }
    encrypt_test!(1);
    encrypt_test!(2);
    encrypt_test!(3);
    encrypt_test!(4);
    encrypt_test!(5);
    encrypt_test!(6);
    encrypt_test!(7);
    encrypt_test!(8);
    encrypt_test!(9);

    let aes1 = Aes::new(Block::ONES);
    let mut blks = [Block::default(); 4];
    blks[1] = Block::ONES;
    blks[3] = Block::ONES;
    Aes::para_encrypt::<2, 2>([aes, aes1], &mut blks);
    assert_eq!(
        blks,
        [
            Block::from(0x2E2B34CA59FA4C883B2C8AEFD44BE966),
            Block::from(0x4E668D3ED24773FA0A5A85EAC98C5B3F),
            Block::from(0x2CC9BF3845486489CD5F7D878C25F6A1),
            Block::from(0x79B93A19527051B230CF80B27C21BFBC)
        ]
    );
}
