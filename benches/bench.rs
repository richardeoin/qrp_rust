#![feature(test)]

extern crate qrp;
extern crate test;

use test::Bencher;
use qrp::wspr;
use qrp::jt4;

///
/// WSPR
///
#[bench]
fn bench_wspr(b: &mut Bencher) {
    b.iter(|| {
        let _ = wspr::encode_wspr("AD1AD", "EJ55", 44);
    });
}

///
/// JT4
///
#[bench]
fn bench_jt4(b: &mut Bencher) {
    b.iter(|| {
        let _ = jt4::encode_jt4(" M0XXX EJ55IE");
    });
}
