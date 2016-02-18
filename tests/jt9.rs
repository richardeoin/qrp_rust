extern crate qrp;

use qrp::jt9;

///
/// JT9
///
mod testvectors;
#[test]
fn jt9_testvectors() {
    let n_symbols = 85;
    let mut testvectors = testvectors::parse_wsjtx_testvectors("jt9", 85);

    let mut it_went_wrong = false;

    while let Some((message, vectors)) = testvectors.pop() { // for each testvector
        let test_result: [u8; 85] = jt9::encode_jt9(&message); // encode it ourselves

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
/// JT9
///
#[test]
fn jt9() {
    let _ = jt9::encode_jt9("M0AAA");
}
