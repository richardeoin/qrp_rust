#![feature(test)]

extern crate qrp;
extern crate test;

use test::Bencher;
use qrp::wspr;
use qrp::jt4;
use qrp::jt9;
use qrp::jt65;

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

///
/// JT9
///
#[bench]
fn bench_jt9(b: &mut Bencher) {
    b.iter(|| {
        let _ = jt9::encode_jt9(" M0XXX EJ55IE");
    });
}

///
/// JT65
///
#[bench]
fn bench_jt65(b: &mut Bencher) {
    b.iter(|| {
        let _ = jt65::encode_jt65(" M0XXX EJ55IE");
    });
}
