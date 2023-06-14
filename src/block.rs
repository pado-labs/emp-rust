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

        // polynomial multiply
        let z = vdupq_n_u8(0);
        let r0 = pmull::<0, 0>(h, y);
        let r1 = pmull::<1, 1>(h, y);
        let t0 = pmull::<0, 1>(h, y);
        let t1 = pmull::<1, 0>(h, y);
        let t0 = veorq_u8(t0, t1);
        let t1 = vextq_u8(z, t0, 8);
        let r0 = veorq_u8(r0, t1);
        let t1 = vextq_u8(t0, z, 8);
        let r1 = veorq_u8(r1, t1);

        (Block(r0), Block(r1))
    }

    #[inline]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn clmul_unsafe(self, x: &Self) -> (Block, Block) {
        unsafe {
            let t = self.0;
            let y = x.0;
            let zero = _mm_clmulepi64_si128(t, y, 0x00);
            let one = _mm_clmulepi64_si128(t, y, 0x10);
            let two = _mm_clmulepi64_si128(t, y, 0x01);
            let three = _mm_clmulepi64_si128(t, y, 0x11);
            let tmp = _mm_xor_si128(one, two);
            let ll = _mm_slli_si128(tmp, 8);
            let rl = _mm_srli_si128(tmp, 8);
            let t = _mm_xor_si128(zero, ll);
            let y = _mm_xor_si128(three, rl);
            (Block(t), Block(y))
        }
        // let h = self.0;
        // let y = x.0;

        // let h0 = h;
        // let h1 = _mm_shuffle_epi32(h, 0x0E);
        // let h2 = _mm_xor_si128(h0, h1);
        // let y0 = y;

        // // Multiply values partitioned to 64-bit parts
        // let y1 = _mm_shuffle_epi32(y, 0x0E);
        // let y2 = _mm_xor_si128(y0, y1);
        // let t0 = _mm_clmulepi64_si128(y0, h0, 0x00);
        // let t1 = _mm_clmulepi64_si128(y, h, 0x11);
        // let t2 = _mm_clmulepi64_si128(y2, h2, 0x00);
        // let t2 = _mm_xor_si128(t2, _mm_xor_si128(t0, t1));
        // let v0 = t0;
        // let v1 = _mm_xor_si128(_mm_shuffle_epi32(t0, 0x0E), t2);
        // let v2 = _mm_xor_si128(t1, _mm_shuffle_epi32(t2, 0x0E));
        // let v3 = _mm_shuffle_epi32(t1, 0x0E);

        // (
        //     Block(_mm_unpacklo_epi64(v0, v1)),
        //     Block(_mm_unpacklo_epi64(v2, v3)),
        // )
    }
}

/// Wrapper for the ARM64 `PMULL` instruction.
#[inline(always)]
#[cfg(target_arch = "aarch64")]
unsafe fn pmull<const A_LANE: i32, const B_LANE: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    mem::transmute(vmull_p64(
        vgetq_lane_u64(vreinterpretq_u64_u8(a), A_LANE),
        vgetq_lane_u64(vreinterpretq_u64_u8(b), B_LANE),
    ))
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
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha12Rng;

    let mut rng = ChaCha12Rng::from_entropy();
    let a: [u8; 16] = rng.gen();
    let b: [u8; 16] = rng.gen();

    let a = Block::new(&a);
    let b: Block = Block::new(&b);

    let d: [u8; 16] = a.into();
    let e: u128 = b.into();
    a.clmul(&b);
    let c = a ^ b;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{:?}", d);
    println!("{:X}", e);

    let x: u128 = 0x7b5b54657374566563746f725d53475d;
    let y: u128 = 0x48692853686179295b477565726f6e5d;
    let x = Block::from(x);
    let y = Block::from(y);
    println!("{}", x);
    println!("{}", y);
    let (res1, res2) = x.clmul(&y);
    println!("{}", res1);
    println!("{}", res2);
}
