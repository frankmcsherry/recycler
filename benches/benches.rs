#![feature(test)]

extern crate recycler;
extern crate test;

use test::Bencher;
use recycler::*;

#[bench]
fn recycler_vec_vec_str(bencher: &mut Bencher) {
    let mut r1 = make_recycler::<Vec<Vec<String>>>();
    bencher.iter(|| {
        let v = { // scope the borrow of r1
            let (mut v1, r2) = r1.new();
            for _ in 0..10 {
                let (mut v2, r3) = r2.new();
                for _ in 0..10 {
                    v2.push(r3.new_from("test!"));
                }
                v1.push(v2);
            }
            v1
        };
        r1.recycle(v);
    });
}

#[bench]
fn allocate_vec_vec_str(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut v1 = Vec::with_capacity(10);
        for _ in 0..10 {
            let mut v2 = Vec::with_capacity(10);
            for _ in 0..10 {
                v2.push(("test!").to_owned());
            }
            v1.push(v2);
        }
        v1
    });
}

#[bench]
fn recreate_vec_vec_str(bencher: &mut Bencher) {
    let mut recycler = make_recycler::<Vec<Vec<String>>>();
    let data = vec![vec!["test!".to_owned(); 10]; 10];
    bencher.iter(|| {
        let record = recycler.recreate(&data);
        recycler.recycle(record);
    });
}

#[bench]
fn clone_vec_vec_str(bencher: &mut Bencher) {
    let data = vec![vec!["test!".to_owned(); 10]; 10];
    bencher.iter(|| {
        data.clone()
    });
}

#[bench]
fn recycler_vec_vec_u64(bencher: &mut Bencher) {
    let mut r1 = make_recycler::<Vec<Vec<u64>>>();
    bencher.iter(move || {
        let v = { // scope the borrow of r1
            let (mut v1, r2) = r1.new();
            for _ in 0..10 {
                let (mut v2, _r3) = r2.new();
                for _ in 0..10 {
                    v2.push(0u64);
                }
                v1.push(v2);
            }
            v1
        };
        r1.recycle(v);
    });
}

#[bench]
fn allocate_vec_vec_u64(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut v1 = Vec::with_capacity(10);
        for _ in 0..10 {
            let mut v2 = Vec::with_capacity(10);
            for _ in 0..10 {
                v2.push(0u64);
            }
            v1.push(v2);
        }
        v1
    });
}

#[bench]
fn recreate_vec_vec_u64(bencher: &mut Bencher) {
    let mut recycler = make_recycler::<Vec<Vec<u64>>>();
    let data = vec![vec![0u64; 10]; 10];
    bencher.iter(|| {
        let record = recycler.recreate(&data);
        recycler.recycle(record);
    });
}

#[bench]
fn clone_vec_vec_u64(bencher: &mut Bencher) {
    let data = vec![vec![0u64; 10]; 10];
    bencher.iter(|| {
        data.clone()
    });
}
