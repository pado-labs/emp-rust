use core::mem;
use std::{
    fmt::{Debug, Display},
    ops::BitXor,
};

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Block(pub uint8x16_t);
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Block(pub __m128i);

impl BitXor for Block {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self::Output {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            Self(veorq_u8(self.0, other.0))
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            Self(_mm_xor_si128(self.0, other.0))
        }
    }
}

impl Block {
    #[inline]
    pub fn new(bytes: &[u8; 16]) -> Self {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            Self(vld1q_u8(bytes.as_ptr()))
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            // `_mm_loadu_si128` performs an unaligned load
            #[allow(clippy::cast_ptr_alignment)]
            Self(_mm_loadu_si128(bytes.as_ptr() as *const __m128i))
        }
    }

    pub fn clmul(self, x: &Self) -> (Block, Block) {
        unsafe { self.clmul_unsafe(&x) }
    }

    #[inline]
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    unsafe fn clmul_unsafe(self, x: &Self) -> (Block, Block) {
        let h = self.0;
        let y = x.0;
        let z = vdupq_n_u8(0);

        let a_lo = mem::transmute(vget_low_u64(vreinterpretq_u64_u8(h)));
        let a_hi = mem::transmute(vget_high_u64(vreinterpretq_u64_u8(h)));
        let b_lo = mem::transmute(vget_low_u64(vreinterpretq_u64_u8(y)));
        let b_hi = mem::transmute(vget_high_u64(vreinterpretq_u64_u8(y)));

        let tmp3 = mem::transmute(vmull_p64(a_lo, b_lo));
        let tmp4 = mem::transmute(vmull_p64(a_hi, b_lo));
        let tmp5 = mem::transmute(vmull_p64(a_lo, b_hi));
        let tmp6 = mem::transmute(vmull_p64(a_hi, b_hi));

        let tmp4 = veorq_u8(tmp4, tmp5);
        let tmp5 = vextq_u8(z, tmp4, 8);
        let tmp3 = veorq_u8(tmp3, tmp5);
        let tmp4 = vextq_u8(tmp4, z, 8);
        let tmp6 = veorq_u8(tmp6, tmp4);

        (Block(tmp3), Block(tmp6))
    }

    #[inline]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn clmul_unsafe(self, x: &Self) -> (Block, Block) {
        unsafe {
            let h = self.0;
            let y = x.0;

            let tmp3 = _mm_clmulepi64_si128(h, y, 0x00);
            let tmp4 = _mm_clmulepi64_si128(h, y, 0x10);
            let tmp5 = _mm_clmulepi64_si128(h, y, 0x01);
            let tmp6 = _mm_clmulepi64_si128(h, y, 0x11);

            let tmp4 = _mm_xor_si128(tmp4, tmp5);
            let tmp5 = _mm_slli_si128(tmp4, 8);
            let tmp4 = _mm_srli_si128(tmp4, 8);
            let tmp3 = _mm_xor_si128(tmp3, tmp5);
            let tmp6 = _mm_xor_si128(tmp6, tmp4);
            (Block(tmp3), Block(tmp6))
        }
    }
}

impl From<Block> for [u8; 16] {
    #[inline(always)]
    fn from(m: Block) -> [u8; 16] {
        unsafe {
            let b: [u8; 16] = mem::transmute(m);
            b
        }
    }
}

impl From<Block> for u128 {
    #[inline(always)]
    fn from(m: Block) -> u128 {
        unsafe {
            let b: u128 = mem::transmute(m);
            b
        }
    }
}

impl From<u128> for Block {
    #[inline(always)]
    fn from(m: u128) -> Block {
        unsafe {
            let b: Block = mem::transmute(m);
            b
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let block: [u8; 16] = (*self).into();
        for byte in block.iter().rev() {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let block: [u8; 16] = (*self).into();
        for byte in block.iter().rev() {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

#[test]
fn clmul_test() {
    let x: u128 = 0x7b5b54657374566563746f725d53475d;
    let y: u128 = 0x48692853686179295b477565726f6e5d;
    let x = Block::from(x);
    let y = Block::from(y);
    // let _res1: u128 = 0xd857e24982ab861c929633d5d36f0451;
    // let _res2: u128 = 0x1d1e1f2c592e7c45d7946a682e55e763;
    let (res1, res2) = x.clmul(&y);
    println!("{}", res1);
    println!("{}", res2);
}
