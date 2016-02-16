//! JT4 for Rust

use conv;
use interleave;

/// Sync Vector
static SYNC_VECTOR: [u8; 207] = [0,0,0,0,1,1,0,0,0,1,1,0,1,1,0,0,
                                 1,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,
                                 0,0,0,0,0,0,0,0,1,0,1,1,0,1,1,0,
                                 1,0,1,1,1,1,1,0,1,0,0,0,1,0,0,1,
                                 0,0,1,1,1,1,1,0,0,0,1,0,1,0,0,0,
                                 1,1,1,1,0,1,1,0,0,1,0,0,0,1,1,0,
                                 1,0,1,0,1,0,1,0,1,1,1,1,1,0,1,0,
                                 1,0,1,1,0,1,0,1,0,1,1,1,0,0,1,0,
                                 1,1,0,1,1,1,1,0,0,0,0,1,1,0,1,1,
                                 0,0,0,1,1,1,0,1,1,1,0,1,1,1,0,0,
                                 1,0,0,0,1,1,0,1,1,0,0,1,0,0,0,1,
                                 1,1,1,1,1,0,0,1,1,0,0,0,0,1,1,0,
                                 0,0,1,0,1,1,0,1,1,1,1,0,1,0,1];

///
/// Encodes up to 13 characters of plaintext as JT4
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
/// Iterator Jt4Frame, each element is a bit to be convoluted
///
/// Returns 72 bits
///
struct Jt4Frame {
    value: (u32, u32, u32),
    index: u32
}
impl Iterator for Jt4Frame {
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
/// New Jt4Frame
///
fn jt4_frame(plaintext: &str) -> Jt4Frame {
    Jt4Frame { value: encode_plaintext(plaintext), index: 0 }
}



///
/// Encodes a JT4 packet
///
pub fn encode_jt4(plaintext: &str) -> [u8; 207] {

    let mut frame: Jt4Frame = jt4_frame(plaintext); // Frame
    let mut coder: conv::ConvK32R12 = conv::ConvK32R12::new(); // Convolutional code
    let mut encoded: [u8; 206] = [0; 206]; // The output of the convolutional coder
    let mut result: [u8; 207] = [0; 207]; // The result is a 4-FSK sequence

    // convolutional coder
    for i in 0..103 {
        match coder.update(frame.next()) {
            Some((a, b)) => {
                encoded[(i*2)+0] = a;
                encoded[(i*2)+1] = b;
            },
            None => panic!("reached end of convolutional coder early @ {}", i)
        }
    }

    // Start the result with a zero bit to serve as a reference tone
    result[0] = SYNC_VECTOR[0] + 2*0;

    // interleave and combine with sync vector
    for (i,j) in interleave::interleave_seq(206).enumerate() {
        result[(j as usize)+1] = SYNC_VECTOR[(j as usize)+1] + 2*encoded[i]
    }
    result
}
