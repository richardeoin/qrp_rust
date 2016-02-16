extern crate qrp;

#[cfg(test)]
mod tests {
    use super::*;

    use qrp::wspr;
    use qrp::jt4;
    use qrp::math_util;
    use qrp::wsjt_interleave;

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

    ///
    /// WSPR
    ///
    #[test]
    fn check_wspr() {
        let t: [u8; 162] = wspr::encode_wspr("M0AAA", "AA00", 30);
    }

    ///
    /// JT4
    ///
    #[test]
    fn check_jt4() {
        // Test vector from Andy Talbot G4JNT
        // http://g4jnt.com/Coding/JT4_Coding_Process.pdf
        let test: [u8; 207] = jt4::encode_jt4(" G4JNT IO90IV");

        let mut jt4_example: [u8; 207] = [0; 207];
        let jt4_example_hex: [u8; 52] = [0x20, 0xDA, 0x3E, 0x50, 0xCC, 0xAA, 0x2D, 0x20,
                                         0x00, 0x82, 0x65, 0x34, 0xC5, 0xD4, 0x4A, 0xE1,
                                         0x25, 0xF4, 0x06, 0xC0, 0x75, 0x96, 0x18, 0x14,
                                         0x6C, 0xEE, 0x55, 0xC4, 0xC7, 0xBB, 0x37, 0x86,
                                         0xF3, 0xF4, 0xA3, 0x45, 0x29, 0xD9, 0xD9, 0xF2,
                                         0x40, 0xF1, 0x63, 0x03, 0x5F, 0xCB, 0x48, 0x16,
                                         0x8C, 0x71, 0x54, 0xCC];
        let mut i = 0;
        for k in 0..52 {
            let j = jt4_example_hex[k];
            jt4_example[(i*4) + 0] = (j >> 6) & 0x3;
            jt4_example[(i*4) + 1] = (j >> 4) & 0x3;
            jt4_example[(i*4) + 2] = (j >> 2) & 0x3;
            if (i*4) + 3 < 207 {
                jt4_example[(i*4) + 3] = (j >> 0) & 0x3;
            }
            i+=1;
        }

        for i in 0..207 {
            println!("test {} -- example {}", test[i], jt4_example[i]);
            assert_eq!(test[i], jt4_example[i]);
        }
    }

}
