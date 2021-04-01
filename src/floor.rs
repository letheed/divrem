mod div {
    mod signed {
        use core::num::Wrapping;
        use core::ops::Div;

        use crate::DivFloor;

        macro_rules! impl_div_floor_signed {
            ($t:ty, $zero:expr, $one:expr) => {
                impl DivFloor for $t {
                    #[inline]
                    fn div_floor(self, other: Self) -> Self {
                        if self > $zero && other < $zero {
                            ((self - $one) / other) - $one
                        } else if self < $zero && other > $zero {
                            ((self + $one) / other) - $one
                        } else {
                            self / other
                        }
                    }
                }

                impl_forward_ref_binop!(impl DivFloor<$t> for $t { div_floor -> Div::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_floor_signed!($t, 0, 1);
                impl_div_floor_signed!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_div_floor_signed!(i8, i16, i32, i64, i128, isize);
    }

    mod unsigned {
        use core::num::Wrapping;
        use core::ops::Div;

        use crate::DivFloor;

        macro_rules! impl_div_floor_unsigned {
            ($t:ty) => {
                impl DivFloor for $t {
                    #[inline]
                    fn div_floor(self, other: Self) -> Self {
                        self / other
                    }
                }

                impl_forward_ref_binop!(impl DivFloor<$t> for $t { div_floor -> Div::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_floor_unsigned!($t);
                impl_div_floor_unsigned!(Wrapping<$t>);
            )*};
        }

        impl_div_floor_unsigned!(u8, u16, u32, u64, u128, usize);
    }
}

mod rem {
    mod signed {
        use core::num::Wrapping;
        use core::ops::Rem;

        use crate::RemFloor;

        macro_rules! impl_rem_floor_signed {
            ($t:ty, $zero:expr, $one:expr) => {
                impl RemFloor for $t {
                    #[inline]
                    fn rem_floor(self, other: Self) -> Self {
                        if self > $zero && other < $zero {
                            ((self - $one) % other) + other + $one
                        } else if self < $zero && other > $zero {
                            ((self + $one) % other) + other - $one
                        } else {
                            self % other
                        }
                    }
                }

                impl_forward_ref_binop!(impl RemFloor<$t> for $t { rem_floor -> Rem::Output });
            };
            ($($t:ty),*) => {$(
                impl_rem_floor_signed!($t, 0, 1);
                impl_rem_floor_signed!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_rem_floor_signed!(i8, i16, i32, i64, i128, isize);
    }

    mod unsigned {
        use core::num::Wrapping;
        use core::ops::Rem;

        use crate::RemFloor;

        macro_rules! impl_rem_floor_unsigned {
            ($t:ty) => {
                impl RemFloor for $t {
                    #[inline]
                    fn rem_floor(self, other: Self) -> Self {
                        self % other
                    }
                }

                impl_forward_ref_binop!(impl RemFloor<$t> for $t { rem_floor -> Rem::Output });
            };
            ($($t:ty),*) => {$(
                impl_rem_floor_unsigned!($t);
                impl_rem_floor_unsigned!(Wrapping<$t>);
            )*};
        }

        impl_rem_floor_unsigned!(u8, u16, u32, u64, u128, usize);
    }
}

mod divrem {
    mod signed {
        use core::num::Wrapping;

        use crate::{DivRem, DivRemFloor};

        macro_rules! impl_div_rem_floor_signed {
            ($t:ty, $zero:expr, $one:expr) => {
                impl DivRemFloor for $t {
                    #[inline]
                    fn div_rem_floor(self, other: Self) -> (Self, Self) {
                        if self > $zero && other < $zero {
                            let (q, r) = (self - $one).div_rem(other);
                            (q - $one, r + other + $one)
                        } else if self < $zero && other > $zero {
                            let (q, r) = (self + $one).div_rem(other);
                            (q - $one, r + other - $one)
                        } else {
                            self.div_rem(other)
                        }
                    }
                }

                impl_forward_ref_binop!(impl DivRemFloor<$t> for $t { div_rem_floor -> DivRem::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_rem_floor_signed!($t, 0, 1);
                impl_div_rem_floor_signed!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_div_rem_floor_signed!(i8, i16, i32, i64, i128, isize);
    }

    mod unsigned {
        use core::num::Wrapping;

        use crate::{DivRem, DivRemFloor};

        macro_rules! impl_div_rem_floor_unsigned {
            ($t:ty) => {
                impl DivRemFloor for $t {
                    #[inline]
                    fn div_rem_floor(self, other: Self) -> (Self, Self) {
                        self.div_rem(other)
                    }
                }

                impl_forward_ref_binop!(impl DivRemFloor<$t> for $t { div_rem_floor -> DivRem::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_rem_floor_unsigned!($t);
                impl_div_rem_floor_unsigned!(Wrapping<$t>);
            )*};
        }

        impl_div_rem_floor_unsigned!(u8, u16, u32, u64, u128, usize);
    }
}
