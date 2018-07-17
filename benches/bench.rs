#![allow(dead_code)]
#![feature(test)]

extern crate test;
extern crate listpack;

use test::Bencher;

#[bench]
fn bench_append_int(b: &mut Bencher) {
    let mut lp = listpack::Listpack::new();

    b.iter(move || {
        lp.append_int(1);
    });
}

#[bench]
fn bench_append_str(b: &mut Bencher) {
    let mut lp = listpack::Listpack::new();

    b.iter(move || {
        lp.append_str("hi");
    });
}

#[bench]
fn bench_get(b: &mut Bencher) {
    let mut lp = listpack::Listpack::new();
    lp.append_int(1);
    lp.append_int(2);

    b.iter(move || {
        lp.get(lp.first().unwrap());
    });
}