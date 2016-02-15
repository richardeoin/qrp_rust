#![feature(test)]
//! yay tests


extern crate test;

pub mod wspr;
pub mod conv;
pub mod wsjt_interleave;
pub mod math_util;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    ///
    /// Check callsign encoding
    ///
    #[cfg(test)]
    fn check_callsign(cs: &str, val: u32) {
        let c = wspr::encode_callsign(&cs.to_string());
        assert_eq!(c, val)
    }
    #[test]
    fn check_callsign_2nd_digit() {
        check_callsign("M0AAA", 259421940)
    }
    #[test]
    fn check_callsign_2nd_digit_hard() {

    }
    #[test]
    fn check_callsign_3rd_digit() {

    }
    #[test]
    #[should_panic]
    fn check_callsign_too_short() {
        check_callsign("AB", 0)
    }
    #[test]
    #[should_panic]
    fn check_callsign_bad_digit() {
        check_callsign("4XABC", 0) // This may be a real callsign, but not for WSPR
    }

    ///
    /// Check locators
    ///
    #[cfg(test)]
    fn check_locator(loc: &str, val: u32) {
        let l = wspr::encode_locator(&loc.to_string());
        assert_eq!(l, val)
    }
    #[test]
    fn check_locator_aa00() {
        check_locator("aa00", 32220)
    }
    #[test]
    fn check_locator_rr99() {
        check_locator("rr99", 179)
    }
    #[test]
    #[should_panic]
    fn check_locator_bad() {
        check_locator("aaaa", 32220)
    }

    ///
    /// Parity
    ///
    #[cfg(test)]
    fn naive_parity_32(vv: u32) -> u32 {
        let mut v = vv;
        let mut parity: u32 = 0;

        while v > 0 {
            parity = !parity;
            v = v & (v - 1);
        }

        parity & 1
    }
    #[test]
    fn check_parity() {
        assert_eq!(math_util::parity_32(0xFF0100FF), 1);
        assert_eq!(math_util::parity_32(0xFF00AAAA), 0);
        assert_eq!(math_util::parity_32(0x43A8765F), naive_parity_32(0x43A8765F));
    }
    ///
    /// Reversal
    ///
    #[test]
    fn check_reverse() {
        assert_eq!(math_util::reverse_8(0x66), 0x66);
        assert_eq!(math_util::reverse_8(0xE1), 0x87);
        assert_eq!(math_util::reverse_8(0x55), 0xAA);
        assert_eq!(math_util::reverse_8(13), 176);
    }

    ///
    /// Convolutional encoder
    ///
    // #[test]
    // fn check_encoder() {
    //     wspr::encode_wspr("M0AAA", "AA00", 30)
    //}

    ///
    /// Interleaver
    ///
    #[test]
    fn check_interleaver() {
        for (i,j) in wsjt_interleave::interleave_seq(4).enumerate() {
            match i {
                0 => assert_eq!(j, 0),
                1 => assert_eq!(j, 2),
                2 => assert_eq!(j, 1),
                3 => assert_eq!(j, 3),
                _ => assert_eq!(0, 0)
            }
        }
    }

    //
    // Bench wspr
    //
    #[bench]
    fn check_encoder(b: &mut Bencher) {
        b.iter(|| {
            wspr::encode_wspr("AD1AD", "EJ55", 44)
        });
    }
}
