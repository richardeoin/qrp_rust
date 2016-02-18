///
/// Common functions for WSJTX modes (JT4, JT9, JT65)
///

///
/// Encodes up to 13 characters of plaintext
/// Returns a 3-tuple of 28 bits, 28 bits and 15 bits
///
fn encode_plaintext(plaintext: &str) -> (u32, u32, u32) {
    let uplaintext = plaintext.to_uppercase();
    let mut chars = uplaintext.chars();
    let mut pt_array: [u8; 13] = [36; 13]; // All spaces to start

    for i in 0usize..13usize {  // iterate over up to 13 chars
        match chars.next() {
            Some(ch) => {
                pt_array[i] = match ch {
                    '0'...'9' => { // digit
                        ch as u8 - 48
                    }
                    'A'...'Z' => { // alpha
                        ch as u8 - 55
                    },
                    '+' => 37,
                    '-' => 38,
                    '.' => 39,
                    '/' => 40,
                    '?' => 41,
                    _ => 36     // other chars are spaces
                }
            },
            None => break
        }
    }

    // first 5 characters (27 bits)
    let val1: u32 = ((((((((pt_array[0] as u32) * 42) + pt_array[1] as u32) * 42) +
                        pt_array[2] as u32) * 42) + pt_array[3] as u32) * 42) +
                      pt_array[4] as u32;
    // next 5 characters (27 bits)
    let val2: u32 = ((((((((pt_array[5] as u32) * 42) + pt_array[6] as u32) * 42) +
                        pt_array[7] as u32) * 42) + pt_array[8] as u32) * 42) +
                      pt_array[9] as u32;
    // final 3 characters (17 bits)
    let val3: u32 = ((((pt_array[10] as u32) * 42) + pt_array[11] as u32) * 42) +
                      pt_array[12] as u32;
    // msb and 2nd msb of this
    let val3_msb1: u32 = if val3 & (1<<16) > 0 {1} else {0};
    let val3_msb2: u32 = if val3 & (1<<15) > 0 {1} else {0};

    // 3-tuple of 28 bits, 28 bits, 15 bits
    ((val1 * 2) + val3_msb2,    // 28 bits
     (val2 * 2) + val3_msb1,    // 28 bits
     val3 & 0x7fff)             // 15 bits
}

///
/// Iterator WsjtxFrame, each element is a bit to be convoluted
///
/// Returns 72 bits
///
pub struct WsjtxFrame {
    value: (u32, u32, u32),
    index: u32
}
impl Iterator for WsjtxFrame {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        // Get index, increment for next time
        let index = self.index;
        self.index += 1;

        if index < 28 {         // 28 bits MSB-first
            let v = self.value.0;
            self.value.0 <<= 1;
            Some(if (v & (1<<27)) > 0 {1} else {0})

        } else if index < 28+28 { // 28 bits MSB-first
            let v = self.value.1;
            self.value.1 <<= 1;
            Some(if (v & (1<<27)) > 0 {1} else {0})

        } else if index < 28+28+1 { // 1 bit high
            Some(1)                 // denotes plain text

        } else if index < 28+28+1+15 { // 15 bits MSB-first
            let v = self.value.2;
            self.value.2 <<= 1;
            Some(if (v & (1<<14)) > 0 {1} else {0})

        } else {
            None
        }
    }
}
///
/// New WsjtxFrame
///
pub fn wsjtx_frame(plaintext: &str) -> WsjtxFrame {
    WsjtxFrame { value: encode_plaintext(plaintext), index: 0 }
}
