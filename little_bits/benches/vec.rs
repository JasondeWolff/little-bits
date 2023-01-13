#![feature(test)]
extern crate test;
use test::Bencher;

#[path = "../src/maths/maths.rs"] mod maths;
use maths::*;

const BENCH_COUNT: usize = 10_000_000;

#[bench]
fn normal(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..BENCH_COUNT {
            let a = Vector4::<f32>::new(1.0, -1.0, 8.9, 0.0);
            let b = Vector4::<f32>::new(10.0, 10.0, 10.0, 5.0);

            let c = a.dot(b);

            test::black_box(c);
            test::black_box(i);
        }
    })
}

#[bench]
fn global(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..BENCH_COUNT {
            let a = Vector4::<f32>::new(1.0, -1.0, 8.9, 0.0);
            let b = Vector4::<f32>::new(10.0, 10.0, 10.0, 5.0);

            let c = dot(a, b);

            test::black_box(c);
            test::black_box(i);
        }
    })
}