extern crate qrp;

#[cfg(test)]
mod tests {

    use qrp::interleave;

    ///
    /// Convolutional encoder
    ///

    ///
    /// Interleaver
    ///
    #[test]
    fn check_interleaver() {
        for (i,j) in interleave::interleave_seq(4).enumerate() {
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
