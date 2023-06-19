//! Useful utils used in the libraries

/// Pack a bit vector into a byte vecotr.
#[inline(always)]
pub fn pack_bits_to_bytes(bits: &Vec<bool>) -> Vec<u8> {
    let nbytes = (bits.len() - 1) / 8 + 1;
    let mut bytes = vec![0; nbytes];
    for i in 0..nbytes {
        for j in 0..8 {
            if 8 * i + j >= bits.len() {
                break;
            }
            bytes[i] |= (bits[8 * i + j] as u8) << j;
        }
    }
    bytes
}

/// Unpack a byte vector to a bit vector with length size.
#[inline(always)]
pub fn unpack_bytes_to_bits(bytes: &Vec<u8>, size: usize) -> Vec<bool> {
    let mut bits = Vec::<bool>::new();
    for (i, byte) in bytes.iter().enumerate() {
        for j in 0..8 {
            if 8 * i + j >= size {
                break;
            }
            bits.push(((byte >> j) & 1) != 0);
        }
    }
    bits
}

#[test]
fn pack_unpack_test() {
    let n = 10;
    let mut bits = vec![false; n];
    for x in bits.iter_mut() {
        *x = rand::random();
    }
    let bytes = pack_bits_to_bytes(&bits);
    let _bits = unpack_bytes_to_bits(&bytes, n);
    assert_eq!(bits, _bits);
}
