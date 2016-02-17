///
/// Parse test vectors
///

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

static TV_DELIMETER: &'static str =
    "==========================================================================\n";

fn read_testvector(modename: &str) -> String {
    // Create a path to the desired file

    let mut path = PathBuf::from("tests/testvectors");
    path.set_extension(modename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => s,
    }
}

///
/// parse testvector files for wspr modes
///
pub fn parse_wspr_testvectors(modename: &str, channel_symbol_len: usize) -> Vec<(String, Vec<u8>)> {

    // get testvectors
    let testvectors = read_testvector(modename);
    let testvectors_iter = testvectors.split(TV_DELIMETER);

    // results vectors
    let mut results: Vec<(String, Vec<u8>)> = vec![];

    // iterate over them
    for tv in testvectors_iter {

        // get message and decode
        let message_vec: Vec<&str> = tv.split("\n").collect(); // first line
        let message_vec2: Vec<&str> = message_vec[0].rsplit("Message:").collect();
        let decoded_vec: Vec<&str> = tv.rsplit("Decoded message:").collect(); // last line
        let decoded_vec2: Vec<&str> = decoded_vec[0].split("ntype:").collect();

        let message = message_vec2[0].trim();
        let decoded = decoded_vec2[0].trim();

        if message.len() > 0 {      // if we found something (not whitespace)
            // message and decoded string should be equal
            // if not something has gone wrong in our test generator
            assert_eq!(message, decoded);

            // get channel symbols
            let channel_symbols_vec: Vec<&str> = tv.rsplitn(2, "hannel symbols:").collect();
            let channel_symbols_vec2: Vec<&str> = channel_symbols_vec[0].split("Decoded message:").collect();
            let channel_symbols_iter = channel_symbols_vec2[0].split_whitespace();

            // parse to u8s
            let mut channel_symbols: Vec<u8> = vec![];
            for cs in channel_symbols_iter.map(|x: &str| { x.parse::<u8>() }) {
                match cs {
                    Ok(c) => channel_symbols.push(c),
                    Err(_) => continue // Skip elements that don't parse
                }
            }

            // check the length is as expected
            // if not something has gone wrong in our test generator
            assert_eq!(channel_symbol_len, channel_symbols.len());

            results.push((message.to_string(), channel_symbols));
        }
    };

    results
}


///
/// parse testvector files for wsjtx modes
///
pub fn parse_wsjtx_testvectors(modename: &str, channel_symbol_len: usize) -> Vec<(String, Vec<u8>)> {

    // get testvectors
    let testvectors = read_testvector(modename);
    let testvectors_iter = testvectors.split(TV_DELIMETER);

    // results vectors
    let mut results: Vec<(String, Vec<u8>)> = vec![];

    // iterate over them
    for tv in testvectors_iter {

        // get message and decoded
        let message_decoded_vec: Vec<&str> = tv.rsplitn(2, " 1.  ").collect();
        let message_decoded = message_decoded_vec[0];
        let mut message_decoded_iter = message_decoded.split("           ");

        let message = match (message_decoded_iter.next(), message_decoded_iter.next()) {
            (Some(m), Some(d)) => {
                // message and decoded string should be equal
                // if not something has gone wrong in our test generator
                assert_eq!(m.trim(), d.trim());
                m.trim().to_string()
            },
            (_, _) => continue
        };

        // get channel symbols
        let channel_symbols_vec: Vec<&str> = tv.rsplitn(2, "hannel symbols").collect();
        let channel_symbols_iter = channel_symbols_vec[0].split_whitespace();

        // parse to u8s
        let mut channel_symbols: Vec<u8> = vec![];
        for cs in channel_symbols_iter.map(|x: &str| { x.parse::<u8>() }) {
            match cs {
                Ok(c) => channel_symbols.push(c),
                Err(_) => continue // Skip elements that don't parse
            }
        }

        // check the length is as expected
        // if not something has gone wrong in our test generator
        assert_eq!(channel_symbol_len, channel_symbols.len());

        results.push((message, channel_symbols));
    };

    results
}


///
/// check wspr testvector files parse okay
///
#[test]
fn check_parse_wspr_testvectors() {
    let _ = parse_wspr_testvectors("wspr", 162);
}

///
/// check wjst testvector files parse okay
///
#[test]
fn check_parse_wsjtx_testvectors() {
    let _ = parse_wsjtx_testvectors("jt4", 206);
    let _ = parse_wsjtx_testvectors("jt9", 85);
    let _ = parse_wsjtx_testvectors("jt65", 63);
}
