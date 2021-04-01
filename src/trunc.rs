use core::num::Wrapping;
use core::ops::{Div, Rem};

use super::DivRem;

macro_rules! impl_div_rem_trunc {
    ($t:ty) => {
        impl DivRem for $t {
            type Output = (<Self as Div>::Output, <Self as Rem>::Output);

            #[inline]
            fn div_rem(self, other: Self) -> <Self as DivRem>::Output {
                (self / other, self % other)
            }
        }

        impl_forward_ref_binop!(impl DivRem<$t> for $t { div_rem -> Output });
    };
    ($($t:ty),*) => {$(
        impl_div_rem_trunc!($t);
        impl_div_rem_trunc!(Wrapping<$t>);
    )*};
}

impl_div_rem_trunc!(i8, i16, i32, i64, i128, isize);
impl_div_rem_trunc!(u8, u16, u32, u64, u128, usize);
