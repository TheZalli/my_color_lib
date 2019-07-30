//! A helper module for angle types.
//!
//! All operations done for these immediately wrap around, so it is impossible to create
//! a value out of bounds with them

use std::ops::*;
use std::f32::consts::PI as PI32;

use num_traits::{ToPrimitive, NumCast};

use crate::{cuw, Channel};

/// A wrapper type for angles in degrees
#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Deg<T>(pub T);

/// A wrapper type for angles in radians
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Rad(pub f32);

impl Channel for Deg<f32> {
    const INTEGER: bool = false;

    fn ch_max() -> Self { Self(360.0) }
    fn ch_mid() -> Self { Self(180.0) }
    fn ch_zero() -> Self { Self(0.0) }

    fn to_range(self) -> Self {
        let a = self.0 % 360.0;
        if a < 0.0 {
            Self(a + 360.0)
        } else {
            Self(a)
        }
    }
}

macro_rules! impl_int_deg_channel {
    ( $( $type:ty ),* ) => { $(
        impl Channel for Deg<$type> {
            const INTEGER: bool = true;

            fn ch_max() -> Self { Self(360) }
            fn ch_mid() -> Self { Self(180) }
            fn ch_zero() -> Self { Self(0) }

            fn to_range(self) -> Self {
                let a = self.0 % 360;
                if a < 0 {
                    Self(a + 360)
                } else {
                    Self(a)
                }
            }
        }
    )* };
}

impl_int_deg_channel!(i16, i32);

impl Channel for Rad {
    const INTEGER: bool = false;

    fn ch_max() -> Self { Rad(PI32 * 2.0) }
    fn ch_mid() -> Self { Rad(PI32) }
    fn ch_zero() -> Self { Rad(0.0) }

    fn to_range(self) -> Self {
        let a = self.0 % Self::ch_max().0;
        if a < cuw(0) {
            Self(a + Self::ch_max().0)
        } else {
            Self(a)
        }
    }
}

impl<T: NumCast> NumCast for Deg<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        T::from(n).map(Self)
    }
}

impl NumCast for Rad {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_f32().map(Self)
    }
}

macro_rules! impl_to_prim_fns {
    () => (
        fn to_i64(&self) -> Option<i64>     { self.0.to_i64() }
        fn to_u64(&self) -> Option<u64>     { self.0.to_u64() }
        fn to_isize(&self) -> Option<isize> { self.0.to_isize() }
        fn to_i8(&self) -> Option<i8>       { self.0.to_i8() }
        fn to_i16(&self) -> Option<i16>     { self.0.to_i16() }
        fn to_i32(&self) -> Option<i32>     { self.0.to_i32() }
        #[cfg(has_i128)]
        fn to_i128(&self) -> Option<i128>   { self.0.to_i128() }
        fn to_usize(&self) -> Option<usize> { self.0.to_usize() }
        fn to_u8(&self) -> Option<u8>       { self.0.to_u8() }
        fn to_u16(&self) -> Option<u16>     { self.0.to_u16() }
        fn to_u32(&self) -> Option<u32>     { self.0.to_u32() }
        #[cfg(has_i128)]
        fn to_u128(&self) -> Option<u128>   { self.0.to_u128() }
        fn to_f32(&self) -> Option<f32>     { self.0.to_f32() }
        fn to_f64(&self) -> Option<f64>     { self.0.to_f64() }
    );
}

impl<T: ToPrimitive> ToPrimitive for Deg<T> {
    impl_to_prim_fns!();
}

impl ToPrimitive for Rad {
    impl_to_prim_fns!();
}

macro_rules! generic_newtype_from_impls {
    ( $struct_name:ident, $( $num_t:ty ),* ) => { $(
        impl From<$num_t> for $struct_name<$num_t> {
            fn from(n: $num_t) -> Self { Self(n) }
        }

        impl From<$struct_name<$num_t>> for $num_t {
            fn from(angle: $struct_name<$num_t>) -> Self { angle.0 }
        }
    )* };
}

generic_newtype_from_impls!(Deg, i16, i32, f32);

impl From<f32> for Rad {
    fn from(n: f32) -> Self { Self(n) }
}

impl From<Rad> for f32 {
    fn from(angle: Rad) -> Self { angle.0 }
}

macro_rules! impl_deg_ops {
    ( $struct_name:ident;
      $( $trait:ident, $fun:ident, $as_trait:ident, $as_fun:ident );*
    ) => { $(
        impl<T> $trait for $struct_name<T>
            where T: $trait<Output=T>, Self: Channel
        {
            type Output = Self;
            fn $fun(self, rhs: Self) -> Self {
                (Self((self.0).$fun(rhs.0))).to_range()
            }
        }

        impl<T> $as_trait for $struct_name<T>
            where T: $as_trait, Self: Channel
        {
            fn $as_fun(&mut self, rhs: Self) {
                (self.0).$as_fun(rhs.0);
            }
        }
    )* };
}

macro_rules! impl_rad_ops {
    ( $struct_name:ident;
      $( $trait:ident, $fun:ident, $as_trait:ident, $as_fun:ident );*
    ) => { $(
        impl $trait for $struct_name
            where Self: Channel
        {
            type Output = Self;
            fn $fun(self, rhs: Self) -> Self {
                (Self((self.0).$fun(rhs.0))).to_range()
            }
        }

        impl $as_trait for $struct_name
            where Self: Channel
        {
            fn $as_fun(&mut self, rhs: Self) {
                (self.0).$as_fun(rhs.0);
            }
        }
    )* };
}

impl_deg_ops!(Deg;
    Add, add, AddAssign, add_assign;
    Sub, sub, SubAssign, sub_assign;
    Mul, mul, MulAssign, mul_assign;
    Div, div, DivAssign, div_assign;
    Rem, rem, RemAssign, rem_assign
);

impl_rad_ops!(Rad;
    Add, add, AddAssign, add_assign;
    Sub, sub, SubAssign, sub_assign;
    Mul, mul, MulAssign, mul_assign;
    Div, div, DivAssign, div_assign;
    Rem, rem, RemAssign, rem_assign
);