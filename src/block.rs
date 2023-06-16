use core::mem;
use std::{
    fmt::{Debug, Display},
    ops::{BitXor, Mul},
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

impl Block {
    #[inline]
    pub fn new(bytes: &[u8; 16]) -> Self {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            Self(vld1q_u8(bytes.as_ptr()))
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            Self(_mm_loadu_si128(bytes.as_ptr() as *const __m128i))
        }
    }

    #[inline(always)]
    pub fn clmul(self, x: &Self) -> (Self, Self) {
        unsafe { self.clmul_unsafe(x) }
    }

    #[inline]
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    unsafe fn clmul_unsafe(self, x: &Self) -> (Self, Self) {
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

    pub fn gfmul(self, x: &Self) -> Self {
        let (a, b) = self.clmul(x);
        reduce(a, b)
    }
}

#[inline(always)]
pub fn reduce(x: Block, y: Block) -> Block {
    unsafe { reduce_unsafe(x, y) }
}

#[inline]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "pclmulqdq")]
unsafe fn reduce_unsafe(x: Block, y: Block) -> Block {
    use std::env::temp_dir;

    let tmp3 = x.0;
    let tmp6 = y.0;
    let xmmmask = _mm_setr_epi32(0xffffffff, 0x00, 0x00, 0x00);
    let tmp7 = _mm_srli_epi32(tmp6, 31);
    let tmp8 = _mm_srli_epi32(tmp6, 30);
    let tmp9 = _mm_srli_epi32(tmp6, 25);

    let tmp7 = _mm_xor_si128(tmp7, tmp8);
    let tmp7 = _mm_xor_si128(tmp7, tmp9);

    let tmp8 = _mm_shuffle_epi32(tmp7, 147);

    let tmp7 = _mm_and_si128(xmmmask, tmp8);
    let tmp8 = _mm_andnot_si128(xmmmask, tmp8);
    let tmp3 = _mm_xor_si128(tmp3, tmp8);
    let tmp6 = _mm_xor_si128(tmp6, tmp7);

    let tmp10 = _mm_slli_epi32(tmp6, 1);
    let tmp3 = _mm_xor_si128(tmp3, tmp10);
    let tmp11 = _mm_slli_epi32(tmp6, 2);
    let tmp3 = _mm_xor_si128(tmp3, tmp11);
    let tmp12 = _mm_slli_epi32(tmp6, 7);
    let tmp3 = _mm_xor_si128(tmp3, tmp12);

    Block(_mm_xor_si128(tmp3, tmp6))
}

#[inline]
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn reduce_unsafe(x: Block, y: Block) -> Block {
    let tmp3 = x.0;
    let tmp6 = y.0;
    let xmmmask = vreinterpretq_u8_u32(vld1q_u32([0xffffffff, 0x0, 0x0, 0x0].as_ptr()));
    let tmp7 = vreinterpretq_u8_u32(vshlq_u32(vreinterpretq_u32_u8(tmp6), vdupq_n_s32(-31)));
    let tmp8 = vreinterpretq_u8_u32(vshlq_u32(vreinterpretq_u32_u8(tmp6), vdupq_n_s32(-30)));
    let tmp9 = vreinterpretq_u8_u32(vshlq_u32(vreinterpretq_u32_u8(tmp6), vdupq_n_s32(-25)));

    let tmp7 = veorq_u8(tmp7, tmp8);
    let tmp7 = veorq_u8(tmp7, tmp9);

    let tmp8 = vmovq_n_u32(vgetq_lane_u32(vreinterpretq_u32_u8(tmp7), 147 & (0x3)));
    let tmp8 = vsetq_lane_u32(
        vgetq_lane_u32(vreinterpretq_u32_u8(tmp7), (147 >> 2) & (0x3)),
        tmp8,
        1,
    );
    let tmp8 = vsetq_lane_u32(
        vgetq_lane_u32(vreinterpretq_u32_u8(tmp7), (147 >> 4) & (0x3)),
        tmp8,
        2,
    );
    let tmp8 = vreinterpretq_u8_u32(vsetq_lane_u32(
        vgetq_lane_u32(vreinterpretq_u32_u8(tmp7), (147 >> 6) & (0x3)),
        tmp8,
        3,
    ));

    let tmp7 = vandq_u8(xmmmask, tmp8);
    let tmp8 = vbicq_u8(tmp8, xmmmask);
    let tmp3 = veorq_u8(tmp3, tmp8);
    let tmp6 = veorq_u8(tmp6, tmp7);

    let tmp10 = vreinterpretq_u8_u32(vshlq_u32(vreinterpretq_u32_u8(tmp6), vdupq_n_s32(1)));
    let tmp3 = veorq_u8(tmp3, tmp10);
    let tmp11 = vreinterpretq_u8_u32(vshlq_u32(vreinterpretq_u32_u8(tmp6), vdupq_n_s32(2)));
    let tmp3 = veorq_u8(tmp3, tmp11);
    let tmp12 = vreinterpretq_u8_u32(vshlq_u32(vreinterpretq_u32_u8(tmp6), vdupq_n_s32(7)));
    let tmp3 = veorq_u8(tmp3, tmp12);
    Block(veorq_u8(tmp3, tmp6))
}

impl Default for Block {
    #[inline(always)]
    fn default() -> Self {
        Block::from(0u128)
    }
}

impl From<Block> for [u8; 16] {
    #[inline(always)]
    fn from(m: Block) -> [u8; 16] {
        unsafe { mem::transmute(m) }
    }
}

impl From<Block> for u128 {
    #[inline(always)]
    fn from(m: Block) -> u128 {
        unsafe { mem::transmute(m) }
    }
}

impl From<u128> for Block {
    #[inline(always)]
    fn from(m: u128) -> Block {
        unsafe { mem::transmute(m) }
    }
}

impl PartialEq for Block {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        let x: u128 = unsafe { mem::transmute(*self) };
        let y: u128 = unsafe { mem::transmute(*other) };
        x == y
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

impl Mul for Block {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.gfmul(&rhs)
    }
}

#[test]
fn clmul_test() {
    let x: u128 = 0x7b5b54657374566563746f725d53475d;
    let y: u128 = 0x48692853686179295b477565726f6e5d;
    let x = Block::from(x);
    let y = Block::from(y);
    let res1 = Block::from(0xd857e24982ab861c929633d5d36f0451);
    let res2 = Block::from(0x1d1e1f2c592e7c45d7946a682e55e763);
    assert_eq!(x.clmul(&y), (res1, res2));
}

#[test]
fn reduce_test() {
    let x: u128 = 0xd857e24982ab861c929633d5d36f0451;
    let y: u128 = 0x1d1e1f2c592e7c45d7946a682e55e763;
    let x = Block::from(x);
    let y = Block::from(y);
    let z = Block::from(0x040229a09a5ed12e7e4e10da323506d2);
    assert_eq!(z,reduce(x,y));
}
