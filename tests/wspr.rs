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


///
/// WSPR
///
mod testvectors;
#[test]
fn wspr_testvectors() {
    let n_symbols = 162;
    let mut testvectors = testvectors::parse_wspr_testvectors("wspr", 162);

    let mut it_went_wrong = false;

    while let Some((message, vectors)) = testvectors.pop() { // for each testvector
        let mut msplit = message.split_whitespace();

        match (msplit.next(), msplit.next(), msplit.next()) { // Attempt to parse as three parts
            (Some(callsign), Some(locator), Some(p)) => {

                if let Ok(power) = p.parse::<i32>() { // Attempt to parse power
                    let test_result: [u8; 162] = wspr::encode_wspr(&callsign, &locator, power); // encode it ourselves

                    let mut d_symbols: u32 = 0;

                    for i in 0..n_symbols {
                        if test_result[i] != vectors[i] { // symbol doesn't match!
                            println!("index {}: test {} -- reference {}", i, test_result[i], vectors[i]);
                            d_symbols += 1;
                        }
                    }

                    if d_symbols > 0 {
                        println!("");
                        println!("Message failed to pass: {}", message);
                        println!("(Total symbol errors {}/{})", d_symbols, n_symbols);
                        it_went_wrong = true;
                    }
                } else {
                    panic!("Supplied power \"{}\" could not be parsed as i32", p);
                }
            },
            (_, _, _) => {
                panic!("Message \"{}\" supplied by testvector is not valid!", message);
            }
        }
    };

    if it_went_wrong {
        println!("");
        panic!("See failures above");
    }
}
