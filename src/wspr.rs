//! WSPR for Rust

use conv;
use interleave;

/// Sync Vector
static SYNC_VECTOR: [u8; 162] = [1,1,0,0,0,0,0,0,1,0,0,0,1,1,1,0,0,0,1,0,
                                 0,1,0,1,1,1,1,0,0,0,0,0,0,0,1,0,0,1,0,1,
                                 0,0,0,0,0,0,1,0,1,1,0,0,1,1,0,1,0,0,0,1,
                                 1,0,1,0,0,0,0,1,1,0,1,0,1,0,1,0,1,0,0,1,
                                 0,0,1,0,1,1,0,0,0,1,1,0,1,0,1,0,0,0,1,0,
                                 0,0,0,0,1,0,0,1,0,0,1,1,1,0,1,1,0,0,1,1,
                                 0,1,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,1,1,
                                 0,0,0,0,0,0,0,1,1,0,1,0,1,1,0,0,0,1,1,0,
                                 0,0];

///
/// Encodes as callsign as a 28-bit value.
/// Callee should verify callsign is valid, invalid chars will be replaced with
/// zero or space as appropriate.
///
fn encode_callsign_with_offset(callsign: &str, offset: usize) -> u32 {
    let mut chars = callsign.chars();
    let mut cs_array: [u8; 6] = [36; 6]; // All spaces to start

    for i in offset..6usize {   // iterate over callsign array
        match chars.next() {
            Some(ch) => {
                cs_array[i] = match ch {
                    'A'...'Z' if i != 2 => {  // no alpha in 3rd field.
                        ch as u8 - 55
                    },
                    '0'...'9' if i < 3 => {   // no digit after third field.
                        ch as u8 - 48
                    },
                    _ if i == 0 || i > 2 => 36, // normally other chars are spaces
                    _ => 0                      // but zero in no-space fields
                }
            },
            None => break
        }
    }

    // Build up 28-bit value
    (((((((((cs_array[0] as u32 * 36) + cs_array[1] as u32) * 10) +
          cs_array[2] as u32) * 27) + (cs_array[3] as u32 -10)) * 27) +
      (cs_array[4] as u32 -10)) * 27) + (cs_array[5] as u32 -10)
}

///
/// Encodes a callsign as a 28 bit value, per the WSPR spec
///
pub fn encode_callsign(callsign: &str) -> u32 {
    //
    // This function mostly verifies this is a valid callsign
    // that can be encoded in the WSPR callsign field before
    // passing on to encode_callsign_with_offset
    //
    let ucallsign = callsign.to_uppercase();
    let mut chars = ucallsign.chars();
    match (chars.next(), chars.next(), chars.next()) {
        (Some(ch1), Some(ch2), Some(ch3)) => {

            // Must have alpha/digit followed by digit
            match (ch1, ch2, ch3) {
                (_, 'A'...'Z', '0'...'9') | (_, '0'...'9', '0'...'9') => {
                    encode_callsign_with_offset(&ucallsign, 0) // 3rd is digit
                },
                ('A'...'Z', '0'...'9', _) | ('0'...'9', '0'...'9', _) => {
                    encode_callsign_with_offset(&ucallsign, 1) // 2nd is digit
                },
                (_, _, _) => panic!("Callsign must have alpha/digit followed by digit")
            }
        },
        (_,_,_) => panic!("Callsign must have at least 3 characters")
    }
}

///
/// Encodes 4-character locator string like AA..99
///
pub fn encode_locator(locator: &str) -> u32 {
    let ulocator = locator.to_uppercase();
    let mut chars = ulocator.chars();
    match (chars.next(), chars.next(), chars.next(), chars.next()) {
        (Some(ch1), Some(ch2), Some(ch3), Some(ch4)) => {

            // Must have two A-R followed by two 0-9
            match (ch1, ch2, ch3, ch4) {
                ('A'...'R', 'A'...'R', '0'...'9', '0'...'9') => {
                    let loc1 = ch1 as u8 - 65;
                    let loc2 = ch2 as u8 - 65;
                    let loc3 = ch3 as u8 - 48;
                    let loc4 = ch4 as u8 - 48;

                    // Build up 16 bit value
                    ((179 - ((10*loc1 as u32) + loc3 as u32)) * 180) +
                        ((10*loc2 as u32) + loc4 as u32)
                }
                (_, _, _, _) => panic!("Locator must have two A-R followed by two 0-9")
            }
        },
        (_, _, _, _) => panic!("Locator must have at least 4 characters")
    }
}

///
/// Encodes power
///
pub fn encode_power(power_d_b_m: i32) -> u32 {

    if power_d_b_m < 0 || power_d_b_m > 60 {
        panic!("Power must be between 0 and 60 dBm");
    }

    power_d_b_m as u32
}


///
/// Iterator WsprFrame, each element is a bit to be convoluted
///
/// Returns 50 bits
///
struct WsprFrame {
    cs: u32,                    // encoded callsign
    ll: u32,                    // encoded locator
    pp: u32,                     // encoded power
    index: u32
}
impl Iterator for WsprFrame {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        // Get index, increment for next time
        let index = self.index;
        self.index += 1;

        if index < 28 {         // 28 bits callsign MSB-first
            let cs = self.cs;
            self.cs <<= 1;
            Some(if (cs & (1<<27)) > 0 {1} else {0})

        } else if index < 28+15 { // 15 bits locator MSB-first
            let ll = self.ll;
            self.ll <<= 1;
            Some(if (ll & (1<<14)) > 0 {1} else {0})

        } else if index < 28+15+1 { // 1 bit high
            Some(1)

        } else if index < 28+15+1+6 { // 6 bits power MSB-first
            let pp = self.pp;
            self.pp <<= 1;
            Some(if (pp & (1<<5)) > 0 {1} else {0})

        } else {
            None
        }
    }
}
///
/// New WsprFrame
///
fn wspr_frame(callsign: &str, locator: &str, power_d_b_m: i32) -> WsprFrame {
    WsprFrame { cs: encode_callsign(callsign), ll: encode_locator(locator),
                pp: encode_power(power_d_b_m), index: 0 }
}

///
/// Encodes a WSPR packet
///
pub fn encode_wspr(callsign: &str, locator: &str, power_d_b_m: i32) -> [u8; 162] {

    let mut frame: WsprFrame = wspr_frame(callsign, locator, power_d_b_m); // Frame
    let mut coder: conv::ConvK32R12 = conv::ConvK32R12::new(); // Convolutional code
    let mut encoded: [u8; 162] = [0; 162]; // The output of the convolutional coder
    let mut result: [u8; 162] = [0; 162]; // The result is a 4-FSK sequence

    // convolutional coder
    for i in 0..81 {
        match coder.update(frame.next()) {
            Some((a, b)) => {
                encoded[(i*2)+0] = a;
                encoded[(i*2)+1] = b;
            },
            None => panic!("reached end of convolutional coder early @ {}", i)
        }
    }

    // interleave and combine with sync vector
    for (i,j) in interleave::interleave_seq(162).enumerate() {
        result[j as usize] = SYNC_VECTOR[j as usize] + 2*encoded[i]
    }
    result
}
