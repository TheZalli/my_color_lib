use std::ops::{Add, Sub, Mul, Div, Rem};
use std::fmt;

/// sRGB gamma value, used for sRGB decoding and encoding.
pub const GAMMA: f32 = 2.4;

macro_rules! wrapper_struct_conv_impls {
    ($outer:ident, $inner:ty) => {
        impl From<$inner> for $outer {
            fn from(arg: $inner) -> Self {
                $outer :: new(arg)
            }
        }

        impl From<$outer> for $inner {
            fn from(arg: $outer) -> Self {
                arg.0
            }
        }

        impl AsRef<$inner> for $outer {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }

        impl AsMut<$inner> for $outer {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }
    };
}

macro_rules! wrapper_struct_impl_ops {
    ($outer:ty, $inner:ty ; $( $op_trait:ident),+ ; $( $op_fun:ident),+ ) => { $(
        impl $op_trait for $outer {
            type Output = $outer;
            fn $op_fun(self, rhs: Self) -> Self::Output {
                $op_trait :: $op_fun(self.0, rhs.0).into()
            }
        }

        impl $op_trait<$inner> for $outer {
            type Output = $outer;
            fn $op_fun(self, rhs: $inner) -> Self::Output {
                $op_trait :: $op_fun(self.0, rhs).into()
            }
        }

        impl $op_trait<$outer> for $inner {
            type Output = $outer;
            fn $op_fun(self, rhs: $outer) -> Self::Output {
                $op_trait :: $op_fun(self, rhs.0).into()
            }
        }
    )+ };
}

/// Clamps the given value into the inclusive range between the given minimum and maximum.
///
/// If no comparison can be made and the function `PartialOrd::partial_cmp` returns `None`, then
/// this function returns the minimum value.
///
/// If the original value is equal to minimum or maximum, the minimum or maximum value is returned
/// respectively.
#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    use std::cmp::Ordering::*;
    match value.partial_cmp(&max) {
        Some(Less) =>
            match value.partial_cmp(&min) {
                Some(Greater) => value,
                _ => min,
            },
        _ => max
    }
}

/// Gamma encodes a linear value into the sRGB space
pub fn std_gamma_encode(linear: f32) -> f32 {
    const SRGB_CUTOFF: f32 = 0.0031308;
    if linear <= SRGB_CUTOFF {
        linear * 12.92
    } else {
        linear.powf(1.0/GAMMA) * 1.055 - 0.055
    }
}

/// Gamma decodes an sRGB value into the linear space
pub fn std_gamma_decode(encoded: f32) -> f32 {
    const SRGB_INV_CUTOFF: f32 = 0.04045;
    if encoded <= SRGB_INV_CUTOFF {
        encoded / 12.92
    } else {
        ((encoded + 0.055)/1.055).powf(GAMMA)
    }
}

/// Degrees that will always be in the range [0, 360).
///
/// Any value outside that is made to fit the range using modulo.
///
/// Trying to convert NaN and infinity floating point into `Deg` will cause a panic.
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Deg(f32);

impl Deg {
    pub fn inv(self) -> Self {
        Deg(360.0 - self.0)
    }
}

impl Deg {
    fn new(degrees: f32) -> Self {
        if !degrees.is_finite() {
            panic!("`Deg`: Tried to convert NaN or infinite value into degrees!")
        }

        let mut degrees = degrees % 360.0;
        if degrees < 0.0 {
            degrees = degrees + 360.0;
        }
        Deg(degrees)
    }
}

impl Ord for Deg {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Deg {}

impl fmt::Display for Deg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

wrapper_struct_conv_impls!(Deg, f32);
wrapper_struct_impl_ops!(
    Deg, f32;
    Add, Sub, Mul, Div, Rem;
    add, sub, mul, div, rem
);
