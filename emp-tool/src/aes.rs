//! Implement aes128
use aes::{
    cipher::{generic_array::GenericArray, typenum::U16, BlockEncrypt, KeyInit},
    Aes128Enc,
};

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::sse2neon::AES_SBOX;
use crate::Block;
use crate::{
    aeskeygenassist_si128, castps_si128, castsi128_ps, cvtsi128_si32, shuffle_epi32, shuffle_ps,
    xor_si128,
};
use std::mem;

///The AES 128 struct
pub struct Aes(Aes128Enc);

impl Aes {
    /// New a Aes instance with `key`
    pub fn new(key: &Block) -> Self {
        let _key: [u8; 16] = (*key).into();
        Self(Aes128Enc::new_from_slice(&_key).unwrap())
    }

    /// Encrypt one block
    pub fn encrypt_block(&self, blk: &Block) -> Block {
        let mut buf = GenericArray::from(*blk);
        self.0.encrypt_block(&mut buf);
        Block::try_from_slice(buf.as_slice()).unwrap()
    }

    /// Encrypt many blocks
    pub fn encrypt_blocks<const N: usize>(&self, blks: &[Block; N]) -> [Block; N] {
        // blks.iter().map(|x|self.encrypt_block(x)).collect::<Vec<Block>>().try_into().unwrap()
        let mut buf: Vec<GenericArray<u8, U16>> =
            blks.iter().map(|&x| GenericArray::from(x)).collect();

        self.0.encrypt_blocks(&mut buf);

        buf.iter()
            .map(|x| Block::from(*x))
            .collect::<Vec<Block>>()
            .try_into()
            .unwrap()
    }
}

///AES related to EMP
pub struct AesEmp {
    rd_key: [Block; 11],
    rounds: usize,
}

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

macro_rules! expand_assist_arm {
    ($v1:expr,$v2:expr,$v3:expr,$v4:expr, $sc:expr,$ac:expr) => {
        $v2 = aeskeygenassist_si128!($v4, $ac);
        $v3 = castps_si128!(shuffle_ps!(castsi128_ps!($v3), castsi128_ps!($v1), 16));
        $v1 = xor_si128!($v1, $v3);
        $v3 = castps_si128!(shuffle_ps!(castsi128_ps!($v3), castsi128_ps!($v1), 140));
        $v1 = xor_si128!($v1, $v3);
        $v2 = shuffle_epi32!($v2, $sc);
        $v1 = xor_si128!($v1, $v2);
    };
}

impl AesEmp {
    /// New AES
    #[inline(always)]
    pub fn new(key: Block) -> Self {
        unsafe { AesEmp::aes_init(key) }
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
        Self {
            rd_key: kp,
            rounds: 10,
        }
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
        Self {
            rd_key: kp,
            rounds: 10,
        }
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
        let mut ctxt = _mm_xor_si128(blk.0, self.rd_key[0].0);
        for i in 1..self.rounds {
            ctxt = _mm_aesenc_si128(ctxt, self.rd_key[i].0);
        }

        ctxt = _mm_aesenclast_si128(ctxt, self.rd_key[self.rounds].0);
        Block(ctxt)
    }

    #[inline]
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "aes")]
    unsafe fn encrypt_backend(&self, blk: Block) -> Block {
        // let mut ctxt = blk.0;
        // for i in 0..self.rounds {
        //     ctxt = vaesmcq_u8(vaeseq_u8(ctxt, self.rd_key[i].0));
        // }

        Block::default()
    }

    /// Encrypt many blocks
    #[inline(always)]
    pub fn encrypt_many_blocks<const N: usize>(&self, blks: &[Block; N]) -> [Block; N] {
        unsafe { self.encrypt_many_backend::<N>(blks) }
    }

    #[inline]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "aes")]
    unsafe fn encrypt_many_backend<const N: usize>(&self, blks: &[Block; N]) -> [Block; N] {
        let mut ctxt = [_mm_setzero_si128(); N];
        for i in 0..N {
            ctxt[i] = _mm_xor_si128(ctxt[i], self.rd_key[0].0);
        }

        for j in 1..self.rounds {
            for i in 0..N {
                ctxt[i] = _mm_aesenc_si128(ctxt[i], self.rd_key[j].0);
            }
        }

        for i in 0..N {
            ctxt[i] = _mm_aesenclast_si128(ctxt[i], self.rd_key[self.rounds].0);
        }

        ctxt.iter()
            .map(|&x| Block(x))
            .collect::<Vec<Block>>()
            .try_into()
            .unwrap()
    }

    #[inline]
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "aes")]
    unsafe fn encrypt_many_backend<const N: usize>(&self, blks: &[Block; N]) -> [Block; N] {
        let mut ctxt = [vdupq_n_u8(0); N];
        ctxt.iter()
            .map(|&x| Block(x))
            .collect::<Vec<Block>>()
            .try_into()
            .unwrap()
    }
}

#[test]
fn aes_new_test() {
    let aes = AesEmp::new(Block::default());
    let c = aes.encrypt_block(Block::default());
    println!("{}", c);
}
