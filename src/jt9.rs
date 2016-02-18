//! JT4 for Rust

use wsjtx;
use conv;
use interleave;
use bithacks;

/// Sync Vector
static SYNC_VECTOR:[u8; 85]=[1,1,0,0,1,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,
                             0,0,1,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,
                             0,0,0,0,0,0,0,0,0,0,1,1,0,0,1,0,0,0,0,1,
                             0,0,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,
                             0,0,1,0,1];

///
/// Encodes a JT9 packet
///
pub fn encode_jt9(plaintext: &str) -> [u8; 85] {

    let mut frame: wsjtx::WsjtxFrame = wsjtx::wsjtx_frame(plaintext); // Frame
    let mut coder: conv::ConvK32R12 = conv::ConvK32R12::new(); // Convolutional code
    let mut encoded: [u8; 206] = [0; 206]; // The output of the convolutional coder
    let mut interleaved: [u8; 206] = [0; 206]; // interleaved bits
    let mut result: [u8; 85] = [0; 85]; // The result is a 9-FSK sequence

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

    // interleave
    for (i,j) in interleave::interleave_seq(206).enumerate() {
        interleaved[j as usize] = encoded[i as usize];
    }
    let mut il_iter = interleaved.iter();

    // generate output symbols
    for i in 0..85 {
        if SYNC_VECTOR[i as usize] == 0 { // it's a data symbol
            // get 3 bits from interleaver and pack them
            let packed = match (il_iter.next(), il_iter.next(), il_iter.next()) {
                (Some(a), Some(b), Some(c)) => {
                    (a&1) << 2 | (b&1) << 1 | (c&1)
                },
                (Some(a), Some(b), _) => { // last two bits
                    (a&1) << 2 | (b&1) << 1
                },
                (_,_,_) => panic!("bits from interleaver used up early @ {}", i)
            };
            // symbol from 1-8
            result[i] = bithacks::gray_code_8(packed) + 1;
        } // other symbols are zeros for sync
    }

    result
}
