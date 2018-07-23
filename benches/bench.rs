#![allow(dead_code)]
#![feature(test)]

extern crate test;
extern crate listpack;

use test::Bencher;

use listpack::raw::*;

#[bench]
fn bench_append_into_val(b: &mut Bencher) {
    let mut lp = listpack::Listpack::new();

    b.iter(move || {
        lp.append(1);
    });
}

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
        lp.append("hello there");
    });
}

#[bench]
fn bench_get_first(b: &mut Bencher) {
    let mut lp = listpack::raw::new(listpack::raw::ALLOCATOR);
    lp = append(listpack::raw::ALLOCATOR, lp, Value::Int(1)).unwrap();
    lp = append(listpack::raw::ALLOCATOR, lp, Value::Int(2)).unwrap();

    b.iter(move || {
        get(first(lp).unwrap());
    });
}

#[bench]
fn bench_get_last(b: &mut Bencher) {
    let mut lp = listpack::raw::new(listpack::raw::ALLOCATOR);
    lp = append(listpack::raw::ALLOCATOR, lp, Value::Int(1)).unwrap();
    lp = append(listpack::raw::ALLOCATOR, lp, Value::Int(2)).unwrap();

    b.iter(move || {
        get(last(lp).unwrap());
    });
}