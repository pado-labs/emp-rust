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

use crate::Block;

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

// macro_rules! expand_assist_arm {
//     ($v1:ident,$v2:ident,$v3:ident,$sc:expr,$ac:expr) => {
//         $v2 = aeskeygenassist_si128!($v4, $ac);
//         $v3 =
//     };
// }

impl AesEmp {
    /// New AES
    pub fn new(key: Block) -> Self {
        unsafe { AesEmp::aes_init(key) }
    }

    #[inline]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "aes",enable = "sse2")]
    unsafe fn aes_init(key: Block) -> Self {
        let mut kp = [Block::default(); 11];
        kp[0] = key;
        let mut x0 = key.0;
        let mut x1 = _mm_setzero_si128();
        let mut x2 = _mm_setzero_si128();

        expand_assist_x86!(x0, x1, x2, x0, 255, 1);
        kp[1] = Block(x0);

        expand_assist_x86!(x0, x1, x2, x0, 255, 2);
        kp[2] = Block(x0);

        expand_assist_x86!(x0, x1, x2, x0, 255, 4);
        kp[3] = Block(x0);

        expand_assist_x86!(x0, x1, x2, x0, 255, 8);
        kp[4] = Block(x0);

        expand_assist_x86!(x0, x1, x2, x0, 255, 16);
        kp[5] = Block(x0);

        expand_assist_x86!(x0, x1, x2, x0, 255, 32);
        kp[6] = Block(x0);

        expand_assist_x86!(x0, x1, x2, x0, 255, 64);
        kp[7] = Block(x0);

        expand_assist_x86!(x0, x1, x2, x0, 255, 128);
        kp[8] = Block(x0);

        expand_assist_x86!(x0, x1, x2, x0, 255, 27);
        kp[9] = Block(x0);

        expand_assist_x86!(x0, x1, x2, x0, 255, 54);
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
        Self {
            rd_key: [Block::default(); 11],
            rounds: 10,
        }
    }
}
