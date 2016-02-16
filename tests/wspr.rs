extern crate qrp;

use qrp::wspr;

pub fn check_callsign(cs: &str, val: u32) {
    let c = wspr::encode_callsign(&cs.to_string());
    assert_eq!(c, val)
}
pub fn check_locator(loc: &str, val: u32) {
    let l = wspr::encode_locator(&loc.to_string());
    assert_eq!(l, val)
}


#[cfg(test)]
mod tests {
    use super::*;

    use qrp::wspr;

    ///
    /// Check callsign encoding
    ///
    #[test]
    fn wspr_callsign() {
        check_callsign("M0AAA", 259421940);
    }
    #[test]
    #[should_panic]
    fn wspr_callsign_should_panic() {
        check_callsign("AB",    0); // too short
        check_callsign("4XABC", 0); // maybe a real callsign, but not for wspr
    }

    ///
    /// Check locator encoding
    ///
    #[test]
    fn wspr_locator() {
        check_locator("aa00", 32220);
        check_locator("rr99", 179);
    }
    #[test]
    #[should_panic]
    fn wspr_locator_should_panic() {
        check_locator("aaaa", 32220);
    }

    ///
    /// Overall encoding
    ///
    #[test]
    fn wspr() {
        let _ = wspr::encode_wspr("M0AAA", "AA00", 30);
    }
}
