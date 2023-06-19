use core::mem;
use std::{
    fmt::{Debug, Display},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign},
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
    #[inline(always)]
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

    #[inline(always)]
    pub fn gfmul(self, x: &Self) -> Self {
        let (a, b) = self.clmul(x);
        Block::reduce(&a, &b)
    }

    #[inline(always)]
    pub fn reduce(x: &Block, y: &Block) -> Block {
        unsafe { Block::reduce_unsafe(x, y) }
    }

    #[inline]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe fn reduce_unsafe(x: &Block, y: &Block) -> Block {
        let tmp3 = x.0;
        let tmp6 = y.0;
        let xmmmask = _mm_setr_epi32(-1, 0x0, 0x0, 0x0);
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
    unsafe fn reduce_unsafe(x: &Block, y: &Block) -> Block {
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

    #[inline(always)]
    pub fn get_lsb(&self) -> bool {
        let x = u128::from(*self);
        (x & 1) == 1
    }

    #[inline(always)]
    pub fn set_lsb(&mut self) {
        *self = (*self) | Block::from(1u128);
    }

    #[inline(always)]
    pub fn set_bit(self, pos: u64) -> Self {
        assert!(pos < 128);
        let x = 1u128 << pos;
        self | Block::from(x)
    }

    #[inline(always)]
    pub fn try_from_slice(bytes_slice: &[u8]) -> Option<Self> {
        if bytes_slice.len() != 16 {
            return None;
        }
        let mut bytes: [u8; 16] = [0; 16];
        bytes[..16].clone_from_slice(&bytes_slice[..16]);
        Some(Block::new(&bytes))
    }

    #[inline(always)]
    pub fn inn_prdt_no_red(a: &Vec<Block>, b: &Vec<Block>) -> (Block, Block) {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .fold((Block::default(), Block::default()), |acc, (x, y)| {
                let t = x.clmul(y);
                (t.0 ^ acc.0, t.1 ^ acc.1)
            })
    }

    #[inline(always)]
    pub fn inn_prdt_red(a: &Vec<Block>, b: &Vec<Block>) -> Block {
        let (x, y) = Block::inn_prdt_no_red(a, b);
        Block::reduce(&x, &y)
    }

    #[inline(always)]
    pub fn pow(&self, exp: u64) -> Self {
        let mut h = *self;
        let mut res = if (exp & 1) == 1 {
            h
        } else {
            Block::from(1u128)
        };

        for i in 1..(64 - exp.leading_zeros()) {
            h = h * h;
            if (exp >> i) & 1 == 1 {
                res = h * res;
            }
        }
        res
    }

    #[inline(always)]
    pub fn inverse(&self) -> Self {
        let mut h = *self;
        let mut res = h;
        for _ in 1..127 {
            h = h * h;
            res = res * h;
        }
        res * res
    }
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
        let x: u128 = (*self).into();
        let y: u128 = (*other).into();
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

impl AsRef<[u8]> for Block {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self as *const Block as *const [u8; 16]) }
    }
}

impl AsMut<[u8]> for Block {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self as *mut Block as *mut [u8; 16]) }
    }
}

impl BitXor for Block {
    type Output = Self;
    #[inline(always)]
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

impl BitXorAssign for Block {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl BitOr for Block {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            Self(vorrq_u8(self.0, rhs.0))
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            Self(_mm_or_si128(self.0, rhs.0))
        }
    }
}

impl BitOrAssign for Block {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitAnd for Block {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            Self(vandq_u8(self.0, rhs.0))
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            Self(_mm_and_si128(self.0, rhs.0))
        }
    }
}

impl BitAndAssign for Block {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl Mul for Block {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        self.gfmul(&rhs)
    }
}

impl MulAssign for Block {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl rand::distributions::Distribution<Block> for rand::distributions::Standard {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Block {
        Block::from(rng.gen::<u128>())
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
    let x = Block::from(0xd857e24982ab861c929633d5d36f0451);
    let y = Block::from(0x1d1e1f2c592e7c45d7946a682e55e763);
    let z = Block::from(0x040229a09a5ed12e7e4e10da323506d2);
    assert_eq!(z, Block::reduce(&x, &y));
}

#[test]
fn gfmul_test() {
    let mut x = Block::from(0x7b5b54657374566563746f725d53475d);
    let y = Block::from(0x48692853686179295b477565726f6e5d);
    let z = Block::from(0x040229a09a5ed12e7e4e10da323506d2);

    assert_eq!(z, x.gfmul(&y));
    assert_eq!(z, x * y);

    x *= y;
    assert_eq!(z, x);
}

#[test]
fn bit_test() {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha12Rng;
    let mut rng = ChaCha12Rng::from_entropy();
    let x: u128 = rng.gen();
    let y: u128 = rng.gen();

    let x: Block = Block::from(x);
    let y: Block = Block::from(y);

    let _x: u128 = x.into();
    let _y: u128 = y.into();

    assert_eq!(Block::from(_x ^ _y), x ^ y);
    assert_eq!(Block::from(_x | _y), x | y);
    assert_eq!(Block::from(_x & _y), x & y);

    let mut z = x;
    z ^= y;
    assert_eq!(Block::from(_x ^ _y), z);

    z = x;
    z |= y;
    assert_eq!(Block::from(_x | _y), z);

    z = x;
    z &= y;
    assert_eq!(Block::from(_x & _y), z);
}

#[test]
fn lsb_test() {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha12Rng;
    let mut rng = ChaCha12Rng::from_entropy();

    let x: u128 = rng.gen();
    let mut y = Block::from(x);
    assert_eq!((x & 1) == 1, y.get_lsb());

    y.set_lsb();
    assert_eq!(y.get_lsb(), true);
}

#[test]
fn inn_prdt_test() {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha12Rng;
    let mut rng = ChaCha12Rng::from_entropy();

    const SIZE: usize = 1000;
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = (Block::default(), Block::default());
    let mut d = Block::default();
    for i in 0..SIZE {
        let r: u128 = rng.gen();
        a.push(Block::from(r));
        let r: u128 = rng.gen();
        b.push(Block::from(r));

        let z = a[i].clmul(&b[i]);
        c.0 = c.0 ^ z.0;
        c.1 = c.1 ^ z.1;

        let x = a[i] * b[i];
        d ^= x;
    }

    assert_eq!(c, Block::inn_prdt_no_red(&a, &b));
    assert_eq!(d, Block::inn_prdt_red(&a, &b));
}

#[test]
fn pow_inverse_test() {
    let one = Block::from(1u128);
    let exp = rand::random::<u64>() % 100;
    let x = Block::from(rand::random::<u128>());
    let mut pow = one;

    for _ in 0..exp {
        pow = pow * x;
    }
    assert_eq!(pow, x.pow(exp));
    assert_eq!(one, x * x.inverse());
}

#[test]
fn to_bytes_test() {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha12Rng;
    let mut rng = ChaCha12Rng::from_entropy();

    let x: Block = rng.gen::<u128>().into();
    assert_eq!(x, Block::try_from_slice(x.as_ref()).unwrap());

    let mut y: Block = rng.gen::<u128>().into();
    let _y = Block::try_from_slice(y.as_mut()).unwrap();
    assert_eq!(y, _y);
}
