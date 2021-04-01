mod div {
    mod signed {
        use core::num::Wrapping;
        use core::ops::Div;

        use crate::DivCeil;

        macro_rules! impl_div_ceil_signed {
            ($t:ty, $zero:expr, $one:expr) => {
                impl DivCeil for $t {
                    #[inline]
                    fn div_ceil(self, other: Self) -> Self {
                        if self > $zero && other > $zero {
                            ((self - $one) / other) + $one
                        } else if self < $zero && other < $zero {
                            ((self + $one) / other) + $one
                        } else {
                            self / other
                        }
                    }
                }

                impl_forward_ref_binop!(impl DivCeil<$t> for $t { div_ceil -> Div::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_ceil_signed!($t, 0, 1);
                impl_div_ceil_signed!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_div_ceil_signed!(i8, i16, i32, i64, i128, isize);
    }

    mod unsigned {
        use core::num::Wrapping;
        use core::ops::Div;

        use crate::DivCeil;

        macro_rules! impl_div_ceil_unsigned {
            ($t:ty, $zero:expr, $one:expr) => {
                impl DivCeil for $t {
                    #[inline]
                    fn div_ceil(self, other: Self) -> Self {
                        if self == $zero {
                            self / other
                        } else {
                            ((self - $one) / other) + $one
                        }
                    }
                }

                impl_forward_ref_binop!(impl DivCeil<$t> for $t { div_ceil -> Div::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_ceil_unsigned!($t, 0, 1);
                impl_div_ceil_unsigned!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_div_ceil_unsigned!(u8, u16, u32, u64, u128, usize);
    }
}

mod rem {
    mod signed {
        use core::num::Wrapping;
        use core::ops::Rem;

        use crate::RemCeil;

        macro_rules! impl_rem_ceil_signed {
            ($t:ty, $zero:expr, $one:expr) => {
                impl RemCeil for $t {
                    #[inline]
                    fn rem_ceil(self, other: Self) -> Self {
                        if self > $zero && other > $zero {
                            ((self - $one) % other) - other + $one
                        } else if self < $zero && other < $zero {
                            ((self + $one) % other) - other - $one
                        } else {
                            self % other
                        }
                    }
                }

                impl_forward_ref_binop!(impl RemCeil<$t> for $t { rem_ceil -> Rem::Output });
            };
            ($($t:ty),*) => {$(
                impl_rem_ceil_signed!($t, 0, 1);
                impl_rem_ceil_signed!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_rem_ceil_signed!(i8, i16, i32, i64, i128, isize);
    }

    mod unsigned {
        use core::num::Wrapping;
        use core::ops::Rem;

        use crate::RemCeil;

        macro_rules! impl_rem_ceil_unsigned {
            ($t:ty, $zero:expr, $one:expr) => {
                impl RemCeil for $t {
                    #[inline]
                    fn rem_ceil(self, other: Self) -> Self {
                        if self == $zero {
                            self % other
                        } else {
                            ((self - $one) % other) - other + $one
                        }
                    }
                }

                impl_forward_ref_binop!(impl RemCeil<$t> for $t { rem_ceil -> Rem::Output });
            };
            ($($t:ty),*) => {$(
                // Modulus is negative or zero since divisor is positive.
                // impl_rem_ceil_unsigned!($t, 0, 1);
                impl_rem_ceil_unsigned!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_rem_ceil_unsigned!(u8, u16, u32, u64, u128, usize);
    }
}

mod divrem {
    mod signed {
        use core::num::Wrapping;

        use crate::{DivRem, DivRemCeil};

        macro_rules! impl_div_rem_ceil_signed {
            ($t:ty, $zero:expr, $one:expr) => {
                impl DivRemCeil for $t {
                    #[inline]
                    fn div_rem_ceil(self, other: Self) -> (Self, Self) {
                        if self > $zero && other > $zero {
                            let (q, r) = (self - $one).div_rem(other);
                            (q + $one, r - other + $one)
                        } else if self < $zero && other < $zero {
                            let (q, r) = (self + $one).div_rem(other);
                            (q + $one, r - other - $one)
                        } else {
                            self.div_rem(other)
                        }
                    }
                }

                impl_forward_ref_binop!(impl DivRemCeil<$t> for $t { div_rem_ceil -> DivRem::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_rem_ceil_signed!($t, 0, 1);
                impl_div_rem_ceil_signed!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_div_rem_ceil_signed!(i8, i16, i32, i64, i128, isize);
    }

    mod unsigned {
        use core::num::Wrapping;

        use crate::{DivRem, DivRemCeil};

        macro_rules! impl_div_rem_ceil_unsigned {
            ($t:ty, $zero:expr, $one:expr) => {
                impl DivRemCeil for $t {
                    #[inline]
                    fn div_rem_ceil(self, other: Self) -> (Self, Self) {
                        if self == $zero {
                            self.div_rem(other)
                        } else {
                            let (q, r) = (self - $one).div_rem(other);
                            (q + $one, r - other + $one)
                        }
                    }
                }

                impl_forward_ref_binop!(impl DivRemCeil<$t> for $t { div_rem_ceil -> DivRem::Output });
            };
            ($($t:ty),*) => {$(
                // Modulus is negative or zero since divisor is positive.
                // impl_div_rem_ceil_unsigned!($t, 0, 1);
                impl_div_rem_ceil_unsigned!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_div_rem_ceil_unsigned!(u8, u16, u32, u64, u128, usize);
    }
}
