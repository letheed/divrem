#![warn(rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![feature(test)]

extern crate test;

use divrem::DivRem;
use divrem::{DivCeil, DivRemCeil, RemCeil};
use divrem::{DivEuclid, DivRemEuclid, RemEuclid};
use divrem::{DivFloor, DivRemFloor, RemFloor};
use test::Bencher;

#[inline]
fn div_floor1(self_: i32, other: i32) -> i32 {
    DivFloor::div_floor(self_, other)
}

#[inline]
fn div_floor2(self_: i32, other: i32) -> i32 {
    match DivRem::div_rem(self_, other) {
        (q, r) if (r > 0 && other < 0) || (r < 0 && other > 0) => q - 1,
        (q, _) => q,
    }
}

#[inline]
fn div_floor3(self_: i32, other: i32) -> i32 {
    match DivRem::div_rem(self_, other) {
        (q, r) if r.signum() == -other.signum() => q - 1,
        (q, _) => q,
    }
}

#[inline]
fn rem_floor1(self_: i32, other: i32) -> i32 {
    RemFloor::rem_floor(self_, other)
}

#[inline]
fn rem_floor2(self_: i32, other: i32) -> i32 {
    let r = self_ % other;
    if (r > 0 && other < 0) || (r < 0 && other > 0) {
        r + other
    } else {
        r
    }
}

#[inline]
fn rem_floor3(self_: i32, other: i32) -> i32 {
    let r = self_ % other;
    if r.signum() == -other.signum() {
        r + other
    } else {
        r
    }
}

#[inline]
fn div_rem_floor1(self_: i32, other: i32) -> (i32, i32) {
    DivRemFloor::div_rem_floor(self_, other)
}

#[inline]
fn div_rem_floor2(self_: i32, other: i32) -> (i32, i32) {
    match DivRem::div_rem(self_, other) {
        (q, r) if (r > 0 && other < 0) || (r < 0 && other > 0) => (q - 1, r + other),
        (q, r) => (q, r),
    }
}

#[inline]
fn div_rem_floor3(self_: i32, other: i32) -> (i32, i32) {
    match DivRem::div_rem(self_, other) {
        (q, r) if r.signum() == -other.signum() => (q - 1, r + other),
        (q, r) => (q, r),
    }
}

#[inline]
fn div_ceil1(self_: i32, other: i32) -> i32 {
    DivCeil::div_ceil(self_, other)
}

#[inline]
fn div_ceil2(self_: i32, other: i32) -> i32 {
    match DivRem::div_rem(self_, other) {
        (q, r) if (r > 0 && other > 0) || (r < 0 && other < 0) => q + 1,
        (q, _) => q,
    }
}

#[inline]
fn div_ceil3(self_: i32, other: i32) -> i32 {
    match DivRem::div_rem(self_, other) {
        (q, r) if r.signum() == other.signum() => q + 1,
        (q, _) => q,
    }
}

#[inline]
fn rem_ceil1(self_: i32, other: i32) -> i32 {
    RemCeil::rem_ceil(self_, other)
}

#[inline]
fn rem_ceil2(self_: i32, other: i32) -> i32 {
    let r = self_ % other;
    if (r > 0 && other > 0) || (r < 0 && other < 0) {
        r - other
    } else {
        r
    }
}

#[inline]
fn rem_ceil3(self_: i32, other: i32) -> i32 {
    let r = self_ % other;
    if r.signum() == other.signum() {
        r - other
    } else {
        r
    }
}

#[inline]
fn div_rem_ceil1(self_: i32, other: i32) -> (i32, i32) {
    DivRemCeil::div_rem_ceil(self_, other)
}

#[inline]
fn div_rem_ceil2(self_: i32, other: i32) -> (i32, i32) {
    match DivRem::div_rem(self_, other) {
        (q, r) if (r > 0 && other > 0) || (r < 0 && other < 0) => (q + 1, r - other),
        (q, r) => (q, r),
    }
}

#[inline]
fn div_rem_ceil3(self_: i32, other: i32) -> (i32, i32) {
    match DivRem::div_rem(self_, other) {
        (q, r) if r.signum() == other.signum() => (q + 1, r - other),
        (q, r) => (q, r),
    }
}

#[inline]
fn div_euclid1(self_: i32, other: i32) -> i32 {
    DivEuclid::div_euclid(self_, other)
}

#[inline]
fn rem_euclid1(self_: i32, other: i32) -> i32 {
    RemEuclid::rem_euclid(self_, other)
}

#[inline]
fn div_rem_euclid1(self_: i32, other: i32) -> (i32, i32) {
    DivRemEuclid::div_rem_euclid(self_, other)
}

macro_rules! bench_loop_inner {
    ($function:expr) => {{
        let mut sum = 0;
        for x in 0..1024 {
            for y in 1..x + 4 {
                sum += $function(x, y);
                sum += $function(x, -y);
                sum += $function(-x, y);
                sum += $function(-x, -y);
            }
        }
        sum
    }};
}

macro_rules! bench_loop {
    ($function:expr,1) => {
        bench_loop_inner!($function)
    };
    ($function:expr,2) => {
        bench_loop_inner!(|x, y| {
            let (q, r) = $function(x, y);
            q + r
        })
    };
}

macro_rules! bench {
    ($bench_name:ident, $function:ident, $n:tt) => {
        #[bench]
        fn $bench_name(b: &mut Bencher) {
            b.iter(|| bench_loop!($function, $n));
        }
    };
}

bench!(bench_div_floor1, div_floor1, 1);
bench!(bench_div_floor2, div_floor2, 1);
bench!(bench_div_floor3, div_floor3, 1);
bench!(bench_rem_floor1, rem_floor1, 1);
bench!(bench_rem_floor2, rem_floor2, 1);
bench!(bench_rem_floor3, rem_floor3, 1);
bench!(bench_div_rem_floor1, div_rem_floor1, 2);
bench!(bench_div_rem_floor2, div_rem_floor2, 2);
bench!(bench_div_rem_floor3, div_rem_floor3, 2);
bench!(bench_div_ceil1, div_ceil1, 1);
bench!(bench_div_ceil2, div_ceil2, 1);
bench!(bench_div_ceil3, div_ceil3, 1);
bench!(bench_rem_ceil1, rem_ceil1, 1);
bench!(bench_rem_ceil2, rem_ceil2, 1);
bench!(bench_rem_ceil3, rem_ceil3, 1);
bench!(bench_div_rem_ceil1, div_rem_ceil1, 2);
bench!(bench_div_rem_ceil2, div_rem_ceil2, 2);
bench!(bench_div_rem_ceil3, div_rem_ceil3, 2);
bench!(bench_div_euclid1, div_euclid1, 1);
bench!(bench_rem_euclid1, rem_euclid1, 1);
bench!(bench_div_rem_euclid1, div_rem_euclid1, 2);

macro_rules! test_algos_eq {
    ($test_name:ident, $functions:expr) => {
        #[test]
        fn $test_name() {
            let mut table = Vec::new();
            for f in &$functions {
                let mut results = Vec::new();
                for x in 0..32 {
                    for y in 1..x + 4 {
                        results.push(f(x, y));
                        results.push(f(x, -y));
                        results.push(f(-x, y));
                        results.push(f(-x, -y));
                    }
                }
                table.push(results);
            }
            assert!(table.iter().zip(table.iter().skip(1)).all(|(a, b)| a == b));
        }
    };
}

test_algos_eq!(test_algos_eq_div_floor, [div_floor1, div_floor2, div_floor3]);
test_algos_eq!(test_algos_eq_rem_floor, [rem_floor1, rem_floor2, rem_floor3]);
test_algos_eq!(test_algos_eq_div_rem_floor, [div_rem_floor1, div_rem_floor2, div_rem_floor3]);

test_algos_eq!(test_algos_eq_div_ceil, [div_ceil1, div_ceil2, div_ceil3]);
test_algos_eq!(test_algos_eq_rem_ceil, [rem_ceil1, rem_ceil2, rem_ceil3]);
test_algos_eq!(test_algos_eq_div_rem_ceil, [div_rem_ceil1, div_rem_ceil2, div_rem_ceil3]);
