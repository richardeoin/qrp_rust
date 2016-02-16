//! Convolutional encoder for K=32, r=1/2

use bithacks;

/// Layland Lushbaugh polynomials
static LL_POLYS: [u32; 2] = [0xF2D05351, 0xE4613C47];

///
/// Convolutional encoder
///
pub struct ConvK32R12 {
    register: u32,              // 32-bit register for K=32
    count: i32
}
impl ConvK32R12 {
    ///
    /// Return a new encoder
    ///
    pub fn new() -> ConvK32R12 {
        ConvK32R12 { register: 0, count: 32 }
    }
    ///
    /// Update the encoder
    ///
    /// Return a two-tuple for rate 1/2
    ///
    pub fn update(&mut self, data: Option<u8>) -> Option<(u8, u8)> {
        // shift up
        self.register <<= 1;

        // new data in
        self.count = match data {
            Some(d) => {
                self.register |= d as u32;
                32              // set count to 32
            },
            None => self.count-1 // count down to empty
        };

        // output
        if self.count > 0 {
            Some((bithacks::parity_32(self.register & LL_POLYS[0]) as u8,
                  bithacks::parity_32(self.register & LL_POLYS[1]) as u8))
        } else { None }
    }
}
