//! Van-der-corput style interleaver for WSJT modes

use bithacks;

///
/// Binary van-der-corput interleave sequence. Max 8-bits
///
pub struct InterleaveSeq {
    index: u8,
    maximum: u8,
}
impl Iterator for InterleaveSeq {
    type Item = u8;
    // Next sequence element
    fn next(&mut self) -> Option<u8> {
        loop {
            if self.index != 0xff {
                let reversed: u8 = bithacks::reverse_8(self.index);
                self.index += 1;

                if reversed < self.maximum {
                    return Some(reversed)
                }
            } else {
                return None
            }
        }
    }
}
///
/// New InterleaveSeq
///
pub fn interleave_seq(max: u8) -> InterleaveSeq {
    InterleaveSeq { index: 0, maximum: max }
}


///
/// Unit tests
///
#[cfg(test)]
mod test {
    use super::*;

    ///
    /// Interleaver
    ///
    #[test]
    fn check_interleaver() {
        for (i,j) in interleave_seq(4).enumerate() {
            match i {
                0 => assert_eq!(j, 0),
                1 => assert_eq!(j, 2),
                2 => assert_eq!(j, 1),
                3 => assert_eq!(j, 3),
                _ => assert_eq!(0, 0)
            }
        }
    }
}
