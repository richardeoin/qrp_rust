//! JT4 for Rust

use wsjtx;
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
/// Encodes a JT4 packet
///
pub fn encode_jt4(plaintext: &str) -> [u8; 207] {

    let mut frame: wsjtx::WsjtxFrame = wsjtx::wsjtx_frame(plaintext); // Frame
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
