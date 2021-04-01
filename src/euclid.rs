mod div {
    mod signed {
        use core::num::Wrapping;
        use core::ops::Div;

        use crate::{DivEuclid, DivRem};

        macro_rules! impl_div_euclid_signed {
            ($t:ty, $zero:expr, $one:expr) => {
                impl DivEuclid for $t {
                    #[inline]
                    fn div_euclid(self, other: Self) -> Self {
                        let (q, r) = self.div_rem(other);
                        if r < $zero {
                            if other > $zero {
                                q - $one
                            } else {
                                q + $one
                            }
                        } else {
                            q
                        }
                    }
                }

                impl_forward_ref_binop!(impl DivEuclid<$t> for $t { div_euclid -> Div::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_euclid_signed!($t, 0, 1);
                impl_div_euclid_signed!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_div_euclid_signed!(i8, i16, i32, i64, i128, isize);
    }

    mod unsigned {
        use core::num::Wrapping;
        use core::ops::Div;

        use crate::DivEuclid;

        macro_rules! impl_div_euclid_unsigned {
            ($t:ty) => {
                impl DivEuclid for $t {
                    #[inline]
                    fn div_euclid(self, other: Self) -> Self {
                        self / other
                    }
                }

                impl_forward_ref_binop!(impl DivEuclid<$t> for $t { div_euclid -> Div::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_euclid_unsigned!($t);
                impl_div_euclid_unsigned!(Wrapping<$t>);
            )*};
        }

        impl_div_euclid_unsigned!(u8, u16, u32, u64, u128, usize);
    }
}

mod rem {
    mod signed {
        use core::num::Wrapping;
        use core::ops::Rem;

        use crate::RemEuclid;

        macro_rules! impl_rem_euclid_signed {
            ($t:ty, $zero:expr, $one:expr) => {
                impl RemEuclid for $t {
                    #[inline]
                    fn rem_euclid(self, other: Self) -> Self {
                        let r = self % other;
                        if r < $zero {
                            if other > $zero {
                                r + other
                            } else {
                                r - other
                            }
                        } else {
                            r
                        }
                    }
                }

                impl_forward_ref_binop!(impl RemEuclid<$t> for $t { rem_euclid -> Rem::Output });
            };
            ($($t:ty),*) => {$(
                impl_rem_euclid_signed!($t, 0, 1);
                impl_rem_euclid_signed!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_rem_euclid_signed!(i8, i16, i32, i64, i128, isize);
    }

    mod unsigned {
        use core::num::Wrapping;
        use core::ops::Rem;

        use crate::RemEuclid;

        macro_rules! impl_rem_euclid_unsigned {
            ($t:ty) => {
                impl RemEuclid for $t {
                    #[inline]
                    fn rem_euclid(self, other: Self) -> Self {
                        self % other
                    }
                }

                impl_forward_ref_binop!(impl RemEuclid<$t> for $t { rem_euclid -> Rem::Output });
            };
            ($($t:ty),*) => {$(
                impl_rem_euclid_unsigned!($t);
                impl_rem_euclid_unsigned!(Wrapping<$t>);
            )*};
        }

        impl_rem_euclid_unsigned!(u8, u16, u32, u64, u128, usize);
    }
}

mod divrem {
    mod signed {
        use core::num::Wrapping;

        use crate::{DivRem, DivRemEuclid};

        macro_rules! impl_div_rem_euclid_signed {
            ($t:ty, $zero:expr, $one:expr) => {
                impl DivRemEuclid for $t {
                    #[inline]
                    fn div_rem_euclid(self, other: Self) -> (Self, Self) {
                        let (q, r) = self.div_rem(other);
                        if r < $zero {
                            if other > $zero {
                                (q - $one, r + other)
                            } else {
                                (q + $one, r - other)
                            }
                        } else {
                            (q, r)
                        }
                    }
                }

                impl_forward_ref_binop!(impl DivRemEuclid<$t> for $t { div_rem_euclid -> DivRem::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_rem_euclid_signed!($t, 0, 1);
                impl_div_rem_euclid_signed!(Wrapping<$t>, Wrapping(0), Wrapping(1));
            )*};
        }

        impl_div_rem_euclid_signed!(i8, i16, i32, i64, i128, isize);
    }

    mod unsigned {
        use core::num::Wrapping;

        use crate::{DivRem, DivRemEuclid};

        macro_rules! impl_div_rem_euclid_unsigned {
            ($t:ty) => {
                impl DivRemEuclid for $t {
                    #[inline]
                    fn div_rem_euclid(self, other: Self) -> (Self, Self) {
                        self.div_rem(other)
                    }
                }

                impl_forward_ref_binop!(impl DivRemEuclid<$t> for $t { div_rem_euclid -> DivRem::Output });
            };
            ($($t:ty),*) => {$(
                impl_div_rem_euclid_unsigned!($t);
                impl_div_rem_euclid_unsigned!(Wrapping<$t>);
            )*};
        }

        impl_div_rem_euclid_unsigned!(u8, u16, u32, u64, u128, usize);
    }
}
