extern crate qrp;

use qrp::jt65;

///
/// JT65
///
mod testvectors;
#[test]
fn jt65_testvectors() {
    let n_symbols = 63;
    let mut testvectors = testvectors::parse_wsjtx_testvectors("jt65", 63);

    let mut it_went_wrong = false;

    while let Some((message, vectors)) = testvectors.pop() { // for each testvector
        // encode it ourselves
        let test_result: [u8; 63] = jt65::encode_jt65_info(&message);

        let mut d_symbols: u32 = 0;

        for i in 0..n_symbols {
            if test_result[i] != vectors[i] { // symbol doesn't match!
                println!("index {}: test {} -- reference {}", i,
                         test_result[i], vectors[i]);
                d_symbols += 1;
            }
        }

        if d_symbols > 0 {
            println!("");
            println!("Message failed to pass: {}", message);
            println!("(Total symbol errors {}/{})", d_symbols, n_symbols);
            it_went_wrong = true;
        }
    };

    if it_went_wrong {
        println!("");
        panic!("See failures above");
    }
}


///
/// JT65
///
#[test]
fn jt65() {
    let _ = jt65::encode_jt65("M0AAA");
}
