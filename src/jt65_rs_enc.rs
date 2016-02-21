//! Reed-Solomon encoder for RS(63, 12)
//! For use in JT65 encoding

///
/// Value of 2^n in GF(64)
///
static EXP_TABLE: [u8; 64] = [0x01,0x02,0x04,0x08,0x10,0x20,0x03,0x06,
                              0x0c,0x18,0x30,0x23,0x05,0x0a,0x14,0x28,
                              0x13,0x26,0x0f,0x1e,0x3c,0x3b,0x35,0x29,
                              0x11,0x22,0x07,0x0e,0x1c,0x38,0x33,0x25,
                              0x09,0x12,0x24,0x0b,0x16,0x2c,0x1b,0x36,
                              0x2f,0x1d,0x3a,0x37,0x2d,0x19,0x32,0x27,
                              0x0d,0x1a,0x34,0x2b,0x15,0x2a,0x17,0x2e,
                              0x1f,0x3e,0x3f,0x3d,0x39,0x31,0x21,0x00];
///
/// Value of log_2(n) in GF(64)
///
static LOG_TABLE: [u8; 64] = [0x3f,0x00,0x01,0x06,0x02,0x0c,0x07,0x1a,
                              0x03,0x20,0x0d,0x23,0x08,0x30,0x1b,0x12,
                              0x04,0x18,0x21,0x10,0x0e,0x34,0x24,0x36,
                              0x09,0x2d,0x31,0x26,0x1c,0x29,0x13,0x38,
                              0x05,0x3e,0x19,0x0b,0x22,0x1f,0x11,0x2f,
                              0x0f,0x17,0x35,0x33,0x25,0x2c,0x37,0x28,
                              0x0a,0x3d,0x2e,0x1e,0x32,0x16,0x27,0x2b,
                              0x1d,0x3c,0x2a,0x15,0x14,0x3b,0x39,0x3a];
///
/// Log coefficients for polynomial division
///
static GEN_POLY: [u8; 52] = [0x2a,0x24,0x39,0x0c,0x09,0x29,0x16,0x15,
                             0x1b,0x27,0x12,0x29,0x34,0x13,0x27,0x15,
                             0x04,0x3b,0x1b,0x0f,0x33,0x0a,0x25,0x33,
                             0x3a,0x24,0x08,0x25,0x25,0x1e,0x0a,0x3a,
                             0x1d,0x30,0x18,0x27,0x00,0x19,0x0c,0x34,
                             0x30,0x20,0x3c,0x37,0x38,0x01,0x1b,0x02,
                             0x0c,0x01,0x32,0x00];
///
/// Fast modulo 63
///
pub fn mod63(mut x: u8) -> u8 {
    while x >= 63 {
        x -= 63;
        x = (x >> 6) + (x & 63);
    }
    x
}

///
/// Reed-Solomon encoder for RS(63, 12)
///
/// This a reversed implementation as used in JT65
///
pub fn rs_n_63_k_12(data: [u8; 12]) -> [u8; 63] {
    let mut parity: [u8; 63] = [0; 63];

    // populate data at far end of array
    for i in 0..12 {
        parity[51+i] = data[i];
    }

    for i in 0..12 { // start from far end
        let feedback = LOG_TABLE[parity[62-i] as usize];

        if feedback != 63 {      // if feedback term is non-zero
            for j in 0..51 {
                parity[(11-i)+j] ^= EXP_TABLE[mod63(feedback + GEN_POLY[j as usize])
                                              as usize];
            }
        }
    }

    // populate data back in
    for i in 0..12 {
        parity[51+i] = data[i];
    }

    parity
}


///
/// Unit tests
///
#[cfg(test)]
mod test {
    use super::*;

    ///
    /// Check fast mod63
    ///
    #[test]
    fn check_mod63() {
        assert_eq!(2%63, mod63(2));
        assert_eq!(22%63, mod63(22));
        assert_eq!(122%63, mod63(122));
        assert_eq!(222%63, mod63(222));
        assert_eq!(255%63, mod63(255));
    }

    ///
    /// Check RS(63, 12)
    ///
    #[test]
    fn check_rs_n_63_k_12() {
        // precalculated reference
        let reference: [u8; 63] = [5,23,22,1,36,23,41,56,57,52,59,3,35,
                                   3,32,26,12,43,41,14,40,34,28,53,13,
                                   3,23,17,55,12,1,63,12,0,32,11,19,22,
                                   43,0,9,61,42,53,16,12,30,46,3,54,17,
                                   0,1,2,3,4,5,6,7,8,9,10,11];
        // current implementation
        let result: [u8; 63] = rs_n_63_k_12([0,1,2,3,4,5,6,7,8,9,10,11]);

        // check equal
        for i in 0..63 {
            println!("index {}: test{} -- reference {} ", i,
                     result[i], reference[i]);
            assert_eq!(result[i], reference[i]);
        }
    }
}
