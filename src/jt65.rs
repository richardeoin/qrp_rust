//! JT65 for Rust

use wsjtx;
use bithacks;
use jt65_rs_enc;

/// Sync Vector
static SYNC_VECTOR:[u8; 126]=[1,0,0,1,1,0,0,0,1,1,1,1,1,1,0,1,0,1,0,0,
                              0,1,0,1,1,0,0,1,0,0,0,1,1,1,0,0,1,1,1,1,
                              0,1,1,0,1,1,1,1,0,0,0,1,1,0,1,0,1,0,1,1,
                              0,0,1,1,0,1,0,1,0,1,0,0,1,0,0,0,0,0,0,1,
                              1,0,0,0,0,0,0,0,1,1,0,1,0,0,1,0,1,1,0,1,
                              0,1,0,1,0,0,1,1,0,0,1,0,0,1,0,0,0,0,1,1,
                              1,1,1,1,1,1];

///
/// Encodes a JT65 packet.
/// Returns information-carrying channel symbols only.
///
pub fn encode_jt65_info(plaintext: &str) -> [u8; 63] {
    let mut frame: wsjtx::WsjtxFrame = wsjtx::wsjtx_frame(plaintext); // Frame
    let mut grouped: [u8; 12] = [0; 12]; // input to reed-solomon coder
    let mut interleaved: [u8; 63] = [0; 63]; // interleaved bits
    let mut result: [u8; 63] = [0; 63]; // The result is a 64-FSK sequence

    // group into six bit symbols
    for i in 0..12 {
        grouped[i] = match (frame.next(), frame.next(), frame.next(),
               frame.next(), frame.next(), frame.next()) {
            (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) => {
                (a<<5)|(b<<4)|(c<<3)|(d<<2)|(e<<1)|f
            },
            (_,_,_,_,_,_) => panic!("reached end of frame early @ {}", i)
        }
    }

    // reed-solomon coder
    let encoded = jt65_rs_enc::rs_n_63_k_12(grouped);

    // interleave for jt65 is a matrix transpose
    // it's actually useless, but it's the standard
    for i in 0..9 {
        for j in 0..7 {
            interleaved[(j*9) + i] = encoded[(i*7) + j];
        }
    }

   // gray code symbols
    for i in 0..63 {
        result[i] = bithacks::gray_code_8(interleaved[i]);
    }

    result
}

///
/// Encodes a JT65 packet.
///
pub fn encode_jt65(plaintext: &str) -> [u8; 126] {
    let info_symbols = encode_jt65_info(plaintext); // Information symbols
    let mut info_symbols_iter = info_symbols.iter();
    let mut result: [u8; 126] = [0; 126]; // Result is a 64-FSK sequence

    // merge with sync vector
    for i in 0..126 {
        if SYNC_VECTOR[i as usize] == 0 { // it's an information symbol
            result[i] = match info_symbols_iter.next() {
                Some(il) => *il + 2, // 2 tones gap from sync tone
                None => panic!("ran out of bits merging with sync vector")
            };
        } // other symbols are zeros for sync
    }

    result
}
