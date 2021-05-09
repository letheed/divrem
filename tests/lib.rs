#![warn(rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![feature(concat_idents)]

#[rustfmt::skip]
const XY: [(i32, i32); 8] =
    [ (8, 3), (8, -3), (-8, 3), (-8, -3)
    , (1, 2), (1, -2), (-1, 2), (-1, -2)
    ];

#[rustfmt::skip]
const QR_TRUNC: [(i32, i32); 8] =
    [ (2, 2), (-2, 2), (-2, -2), (2, -2)
    , (0, 1), (0, 1), (0, -1), (0, -1)
    ];

#[rustfmt::skip]
const QR_FLOOR: [(i32, i32); 8] =
    [ (2, 2), (-3, -1), (-3, 1), (2, -2)
    , (0, 1), (-1, -1), (-1, 1), (0, -1)
    ];

#[rustfmt::skip]
const QR_CEIL: [(i32, i32); 8] =
    [ (3, -1), (-2, 2), (-2, -2), (3, 1)
    , (1, -1), (0, 1), (0, -1), (1, 1)
    ];

#[rustfmt::skip]
const QR_EUCLID: [(i32, i32); 8] =
    [ (2, 2), (-2, 2), (-3, 1), (3, 1)
    , (0, 1), (0, 1), (-1, 1), (1, 1)
    ];

macro_rules! div_rem_functions {
    ($($t:ty),*) => {$(
        fn div_rem(x: $t, y: $t) -> ($t, $t) {
            div_rem_trunc(x, y)
        }

        fn div_rem_trunc(x: $t, y: $t) -> ($t, $t) {
            let q = ((x as f32) / (y as f32)).trunc() as $t;
            let r = x - q * y;
            (q, r)
        }

        fn div_rem_floor(x: $t, y: $t) -> ($t, $t) {
            let q = ((x as f32) / (y as f32)).floor() as $t;
            let r = x - q * y;
            (q, r)
        }

        fn div_floor(x: $t, y: $t) -> $t {
            div_rem_floor(x, y).0
        }

        fn rem_floor(x: $t, y: $t) -> $t {
            div_rem_floor(x, y).1
        }

        fn div_euclid(x: $t, y: $t) -> $t {
            div_rem_euclid(x, y).0
        }

        fn rem_euclid(x: $t, y: $t) -> $t {
            div_rem_euclid(x, y).1
        }
    )*};
}

mod signed {
    use divrem::DivRem;
    use divrem::{DivCeil, DivRemCeil, RemCeil};
    use divrem::{DivEuclid, DivRemEuclid, RemEuclid};
    use divrem::{DivFloor, DivRemFloor, RemFloor};

    fn div_trunc(x: i32, y: i32) -> i32 {
        div_rem_trunc(x, y).0
    }

    fn rem_trunc(x: i32, y: i32) -> i32 {
        div_rem_trunc(x, y).1
    }

    fn div_rem_ceil(x: i32, y: i32) -> (i32, i32) {
        #[allow(clippy::cast_possible_truncation)] // ⌈x/y⌉ is an integer <= |x|.
        let q = (f64::from(x) / f64::from(y)).ceil() as i32;
        let r = x - q * y;
        (q, r)
    }

    fn div_ceil(x: i32, y: i32) -> i32 {
        div_rem_ceil(x, y).0
    }

    fn rem_ceil(x: i32, y: i32) -> i32 {
        div_rem_ceil(x, y).1
    }

    fn div_rem_euclid(x: i32, y: i32) -> (i32, i32) {
        let (mut q, mut r) = (x / y, x % y);
        // Euclid is mod dominant.
        // A unique solution exists for r>=0.
        if r < 0 {
            if y > 0 {
                q -= 1;
                r += y;
            } else {
                q += 1;
                r -= y;
            }
        }
        (q, r)
    }

    div_rem_functions!(i32);

    macro_rules! test_table {
        ($test_name:ident, $table:ident, $variant:ident) => {
            #[test]
            fn $test_name() {
                use super::{$table, XY};

                for (&(x, y), &tqr) in XY.iter().zip(&$table) {
                    let q = concat_idents!(div_, $variant)(x, y);
                    let r = concat_idents!(rem_, $variant)(x, y);
                    let qr = concat_idents!(div_rem_, $variant)(x, y);
                    assert_eq!(tqr, (q, r));
                    assert_eq!(tqr, qr);
                }
            }
        };
    }

    test_table!(test_table_trunc, QR_TRUNC, trunc);
    test_table!(test_table_floor, QR_FLOOR, floor);
    test_table!(test_table_ceil, QR_CEIL, ceil);
    test_table!(test_table_euclid, QR_EUCLID, euclid);

    macro_rules! test {
        ($test_name:ident, $trait:ident, $function:ident) => {
            #[test]
            fn $test_name() {
                for x in 0_i32..32 {
                    for y in 1_i32..x + 4 {
                        assert_eq!($trait::$function(x, y), $function(x, y));
                        assert_eq!($trait::$function(x, -y), $function(x, -y));
                        assert_eq!($trait::$function(-x, y), $function(-x, y));
                        assert_eq!($trait::$function(-x, -y), $function(-x, -y));
                    }
                }
            }
        };
    }

    test!(test_div_rem_trunc, DivRem, div_rem);
    test!(test_div_floor, DivFloor, div_floor);
    test!(test_rem_floor, RemFloor, rem_floor);
    test!(test_div_rem_floor, DivRemFloor, div_rem_floor);
    test!(test_div_euclid, DivEuclid, div_euclid);
    test!(test_rem_euclid, RemEuclid, rem_euclid);
    test!(test_div_rem_euclid, DivRemEuclid, div_rem_euclid);
    test!(test_div_ceil, DivCeil, div_ceil);
    test!(test_rem_ceil, RemCeil, rem_ceil);
    test!(test_div_rem_ceil, DivRemCeil, div_rem_ceil);
}

mod unsigned {
    use std::num::Wrapping;

    use divrem::DivRem;
    use divrem::{DivCeil, DivRemCeil, RemCeil};
    use divrem::{DivEuclid, DivRemEuclid, RemEuclid};
    use divrem::{DivFloor, DivRemFloor, RemFloor};

    fn div_ceil(x: u32, y: u32) -> u32 {
        #[allow(clippy::cast_sign_loss)] // ∀x,y>=0 ⌈x/y⌉>=0.
        #[allow(clippy::cast_possible_truncation)] // ⌈x/y⌉ is an integer <=x.
        let q = (f64::from(x) / f64::from(y)).ceil() as u32;
        q
    }

    fn div_rem_ceil(x: Wrapping<u32>, y: Wrapping<u32>) -> (Wrapping<u32>, Wrapping<u32>) {
        #[allow(clippy::cast_sign_loss)] // ∀n,m>=0 ⌈n/m⌉>=0
        #[allow(clippy::cast_possible_truncation)] // ⌈x/y⌉ is an integer <=x.
        let q = Wrapping((f64::from(x.0) / f64::from(y.0)).ceil() as u32);
        let r = x - q * y;
        (q, r)
    }

    fn rem_ceil(x: Wrapping<u32>, y: Wrapping<u32>) -> Wrapping<u32> {
        div_rem_ceil(x, y).1
    }

    fn div_rem_euclid(x: u32, y: u32) -> (u32, u32) {
        (x / y, x % y)
    }

    div_rem_functions!(u32);

    macro_rules! test {
        ($test_name:ident, $trait:ident, $function:ident) => {
            #[test]
            fn $test_name() {
                for x in 0_u32..32 {
                    for y in 1_u32..x + 4 {
                        assert_eq!($trait::$function(x, y), $function(x, y));
                    }
                }
            }
        };
    }

    test!(test_div_rem_trunc, DivRem, div_rem);
    test!(test_div_floor, DivFloor, div_floor);
    test!(test_rem_floor, RemFloor, rem_floor);
    test!(test_div_rem_floor, DivRemFloor, div_rem_floor);
    test!(test_div_euclid, DivEuclid, div_euclid);
    test!(test_rem_euclid, RemEuclid, rem_euclid);
    test!(test_div_rem_euclid, DivRemEuclid, div_rem_euclid);
    test!(test_div_ceil, DivCeil, div_ceil);

    macro_rules! test_wrap {
        ($test_name:ident, $trait:ident, $function:ident) => {
            #[test]
            fn $test_name() {
                for x in 0_u32..32 {
                    for y in 1_u32..x + 4 {
                        let (x, y) = (Wrapping(x), Wrapping(y));
                        assert_eq!($trait::$function(x, y), $function(x, y));
                    }
                }
            }
        };
    }

    test_wrap!(test_rem_ceil, RemCeil, rem_ceil);
    test_wrap!(test_div_rem_ceil, DivRemCeil, div_rem_ceil);
}
