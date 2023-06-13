use core::mem;
use std::ops::BitXor;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[derive(Clone, Copy)]
#[cfg(target_arch = "aarch64")]
pub struct Block(pub uint8x16_t);
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
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
    pub fn clmul(mut self, x: Self) -> (Block, Block) {
        unsafe { self.clmul_unsafe(&x) }
    }

    #[inline]
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    unsafe fn clmul_unsafe(&mut self, x: &Self) -> (Block, Block) {
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
    unsafe fn clmul_unsafe(self, x: Self) -> (Block, Block) {
        let h = self.0;
        let y = x.0;

        let h0 = h;
        let h1 = _mm_shuffle_epi32(h, 0x0E);
        let h2 = _mm_xor_si128(h0, h1);
        let y0 = y;

        // Multiply values partitioned to 64-bit parts
        let y1 = _mm_shuffle_epi32(y, 0x0E);
        let y2 = _mm_xor_si128(y0, y1);
        let t0 = _mm_clmulepi64_si128(y0, h0, 0x00);
        let t1 = _mm_clmulepi64_si128(y, h, 0x11);
        let t2 = _mm_clmulepi64_si128(y2, h2, 0x00);
        let t2 = _mm_xor_si128(t2, _mm_xor_si128(t0, t1));
        let v0 = t0;
        let v1 = _mm_xor_si128(_mm_shuffle_epi32(t0, 0x0E), t2);
        let v2 = _mm_xor_si128(t1, _mm_shuffle_epi32(t2, 0x0E));
        let v3 = _mm_shuffle_epi32(t1, 0x0E);

        (
            ClmulX86(_mm_unpacklo_epi64(v0, v1)),
            ClmulX86(_mm_unpacklo_epi64(v2, v3)),
        )
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

#[test]
fn platform() {
    #[cfg(target_arch = "x86")]
    println!("x86");
}
