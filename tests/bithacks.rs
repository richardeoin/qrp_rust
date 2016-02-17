extern crate qrp;

use qrp::bithacks;

///
/// Naive implementation of parity check
///
pub fn naive_parity_32(vv: u32) -> u32 {
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
    assert_eq!(bithacks::parity_32(0xFF0100FF), 1);
    assert_eq!(bithacks::parity_32(0xFF00AAAA), 0);
    assert_eq!(bithacks::parity_32(0x43A8765F), naive_parity_32(0x43A8765F));
    assert_eq!(bithacks::parity_32(0xE3334AAA), naive_parity_32(0xE3334AAA));
}
///
/// Reversal of 8-bit byte
///
#[test]
fn bithacks_reverse_8() {
    assert_eq!(bithacks::reverse_8(0x66), 0x66);
    assert_eq!(bithacks::reverse_8(0xE1), 0x87);
    assert_eq!(bithacks::reverse_8(0x55), 0xAA);
    assert_eq!(bithacks::reverse_8(13), 176);
}
