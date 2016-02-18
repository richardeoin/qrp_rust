//! various math utilities

///
/// Computes the parity of a 32-bit word
///
/// see bithacks: https://graphics.stanford.edu/~seander/bithacks.html#ParityParallel
///
/// ```
// use bithacks::parity_32;
//
// assert_eq!(bithacks::parity_32(0xFF01), 1)
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


///
/// Unit tests
///
#[cfg(test)]
mod test {
    use super::*;

    ///
    /// Naive implementation of parity check
    ///
    fn naive_parity_32(vv: u32) -> u32 {
        let mut v = vv;
        let mut parity: u32 = 0;

        while v > 0 {
            parity = !parity;
            v = v & (v - 1);
        }

        parity & 1
    }

    ///
    /// Parity of 32-bit word
    ///
    #[test]
    fn bithacks_parity_32() {
        assert_eq!(parity_32(0xFF0100FF), 1);
        assert_eq!(parity_32(0xFF00AAAA), 0);
        assert_eq!(parity_32(0x43A8765F), naive_parity_32(0x43A8765F));
        assert_eq!(parity_32(0xE3334AAA), naive_parity_32(0xE3334AAA));
    }
    ///
    /// Reversal of 8-bit byte
    ///
    #[test]
    fn bithacks_reverse_8() {
        assert_eq!(reverse_8(0x66), 0x66);
        assert_eq!(reverse_8(0xE1), 0x87);
        assert_eq!(reverse_8(0x55), 0xAA);
        assert_eq!(reverse_8(13), 176);
    }
}
