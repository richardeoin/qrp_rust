//! main


mod interleave;
mod bithacks;

fn main() {
    for (i,j) in interleave::interleave_seq(162).enumerate() {
        println!("i = {} and j = {}",i,j)
    }
}
