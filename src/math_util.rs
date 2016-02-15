//! various math utilities

///
/// Computes the parity of a 32-bit word
///
/// see bithacks: https://graphics.stanford.edu/~seander/bithacks.html#ParityParallel
///
/// ```
// use math_util::parity_32;
//
// assert_eq!(math_util::parity_32(0xFF01), 1)
/// ```
///
pub fn parity_32(vv: u32) -> u32 {
    let vw = vv ^ (vv >> 16);   // Fold over
    let vb = vw ^ (vw >>  8);
    let vn = vb ^ (vb >>  4);

    let vs = vn & 0xf;          // look at lower nibble

    (0x6996 >> vs) & 1          // parity table
}

///
/// Reverse an 8-bit word
///
/// see bithacks: https://graphics.stanford.edu/~seander/bithacks.html#ReverseParallel
///
pub fn reverse_8(vv: u8) -> u8 {
    // swap odd and even bits
    let vo = ((vv >> 1) & 0x55) | ((vv & 0x55) << 1);
    // swap consecutive pairs
    let vc = ((vo >> 2) & 0x33) | ((vo & 0x33) << 2);
    // swap nibbles
    ((vc >> 4) & 0x0F) | ((vc & 0x0F) << 4)
}
