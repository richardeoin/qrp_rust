extern crate qrp;

use qrp::jt4;

///
/// JT4
///
#[test]
fn jt4() {
    // Reference vector from Andy Talbot G4JNT
    // http://g4jnt.com/Coding/JT4_Coding_Process.pdf
    let test: [u8; 207] = jt4::encode_jt4(" G4JNT IO90IV");

    let mut jt4_reference: [u8; 207] = [0; 207];
    let jt4_reference_hex: [u8; 52] = [0x20, 0xDA, 0x3E, 0x50, 0xCC, 0xAA, 0x2D, 0x20,
                                       0x00, 0x82, 0x65, 0x34, 0xC5, 0xD4, 0x4A, 0xE1,
                                       0x25, 0xF4, 0x06, 0xC0, 0x75, 0x96, 0x18, 0x14,
                                       0x6C, 0xEE, 0x55, 0xC4, 0xC7, 0xBB, 0x37, 0x86,
                                       0xF3, 0xF4, 0xA3, 0x45, 0x29, 0xD9, 0xD9, 0xF2,
                                       0x40, 0xF1, 0x63, 0x03, 0x5F, 0xCB, 0x48, 0x16,
                                       0x8C, 0x71, 0x54, 0xCC];
    let mut i = 0;
    for k in 0..52 {
        let j = jt4_reference_hex[k];
        jt4_reference[(i*4) + 0] = (j >> 6) & 0x3;
        jt4_reference[(i*4) + 1] = (j >> 4) & 0x3;
        jt4_reference[(i*4) + 2] = (j >> 2) & 0x3;
        if (i*4) + 3 < 207 {
            jt4_reference[(i*4) + 3] = (j >> 0) & 0x3;
        }
        i+=1;
    }

    for i in 0..207 {
        println!("index {}: test {} -- reference {}", i, test[i], jt4_reference[i]);
        assert_eq!(test[i], jt4_reference[i]);
    }
}
