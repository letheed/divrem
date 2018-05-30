macro_rules! impl_forward_ref_binop {
    (impl $trait:ident < $u:ty > for $t:ty { $method:ident -> $target:ident::Output }) => {
        impl<'a> $trait<$u> for &'a $t {
            #[inline]
            fn $method(self, other: $u) -> <$t as $target<$u>>::Output {
                $trait::$method(*self, other)
            }
        }

        impl<'a> $trait<&'a $u> for $t {
            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $target<$u>>::Output {
                $trait::$method(self, *other)
            }
        }

        impl<'a, 'b> $trait<&'a $u> for &'b $t {
            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $target<$u>>::Output {
                $trait::$method(*self, *other)
            }
        }
    };
    (impl $trait:ident < $u:ty > for $t:ty { $method:ident -> Output }) => {
        impl<'a> $trait<$u> for &'a $t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $trait<$u>>::Output {
                $trait::$method(*self, other)
            }
        }

        impl<'a> $trait<&'a $u> for $t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $trait<$u>>::Output {
                $trait::$method(self, *other)
            }
        }

        impl<'a, 'b> $trait<&'a $u> for &'b $t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $trait<$u>>::Output {
                $trait::$method(*self, *other)
            }
        }
    };
}
