//! main


mod wsjt_interleave;
mod math_util;

fn main() {
    for (i,j) in wsjt_interleave::interleave_seq(162).enumerate() {
        println!("i = {} and j = {}",i,j)
    }
}
