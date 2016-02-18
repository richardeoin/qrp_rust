extern crate qrp;

use qrp::wspr;

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

        // Attempt to parse as three parts
        match (msplit.next(), msplit.next(), msplit.next()) {
            (Some(callsign), Some(locator), Some(p)) => {

                if let Ok(power) = p.parse::<i32>() { // Attempt to parse power
                    // encode it ourselves
                    let test_result: [u8; 162] = wspr::encode_wspr(&callsign,
                                                                   &locator, power);

                    let mut d_symbols: u32 = 0;

                    for i in 0..n_symbols {
                        if test_result[i] != vectors[i] { // symbol doesn't match!
                            //println!("index {}: test {} -- reference {}", i,
                            //         test_result[i], vectors[i]);
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


///
/// WSPR
///
#[test]
fn wspr() {
    let _ = wspr::encode_wspr("M0AAA", "AA00", 30);
}
