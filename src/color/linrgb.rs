use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

use color::*;

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
/// An RGB color with channels normalized between 0 and 1 in the linear space.
pub struct LinRGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    _priv: ()
}

impl LinRGBColor {
    /// Creates a new `LinRGBColor` using the given rgb-values.
    ///
    /// The values are clamped into 0.0 - 1.0 range.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        let f = |x| clamp(x, 0.0, 1.0);
        LinRGBColor { r: f(r), g: f(g), b: f(b), _priv: () }
    }
}

impl Color for LinRGBColor {
    fn srgb(&self) -> SRGBColor {
        let (r, g, b) = self.into();
        (gamma_encode(r), gamma_encode(g), gamma_encode(b)).into()
    }

    fn lin_rgb(&self) -> LinRGBColor { *self }

    fn lin_rgb48(&self) -> LinRGB48Color {
        const MAX: f32 = u16::max_value() as f32;
        let (r, g, b) = self.into();
        ((r * MAX) as u16, (g * MAX) as u16, (b * MAX) as u16).into()
    }
}

impl Add for LinRGBColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.into();
        let (r2, g2, b2) = rhs.into();
        (r1 + r2, g1 + g2, b1 + b2).into()
    }
}

impl Sub for LinRGBColor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.into();
        let (r2, g2, b2) = rhs.into();
        (r1 - r2, g1 - g2, b1 - b2).into()
    }
}

impl Mul<f32> for LinRGBColor {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let (r, g, b) = self.into();
        (r * rhs, g * rhs, b * rhs).into()
    }
}

impl Mul<LinRGBColor> for f32 {
    type Output = LinRGBColor;

    fn mul(self, rhs: LinRGBColor) -> Self::Output {
        let (r, g, b) = rhs.into();
        (self * r, self * g, self * b).into()
    }
}

impl Div<f32> for LinRGBColor {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let (r, g, b) = self.into();
        (r / rhs, g / rhs, b / rhs).into()
    }
}

impl Div<LinRGBColor> for f32 {
    type Output = LinRGBColor;

    fn div(self, rhs: LinRGBColor) -> Self::Output {
        let (r, g, b) = rhs.into();
        (self / r, self / g, self / b).into()
    }
}

impl fmt::Display for LinRGBColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>7.1}%,{:>7.1}%,{:>7.1}%", self.r * 100.0, self.g * 100.0, self.b * 100.0)
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// A 48-bit color with red, green and blue channels in the linear color space.
pub struct LinRGB48Color {
    pub r: u16,
    pub g: u16,
    pub b: u16
}

impl LinRGB48Color {
    /// Create a new sRGB color.
    pub fn new(r: u16, g: u16, b: u16) -> Self { LinRGB48Color { r, g, b } }
}

impl Color for LinRGB48Color {
    fn srgb(&self) -> SRGBColor { self.lin_rgb().srgb() }

    fn lin_rgb(&self) -> LinRGBColor {
        const MAX: f32 = u16::max_value() as f32;
        let (r, g, b) = self.into();
        LinRGBColor::new(r as f32 / MAX, g as f32 / MAX, b as f32 / MAX)
    }

    fn lin_rgb48(&self) -> LinRGB48Color { *self }
}

impl Add for LinRGB48Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.into();
        let (r2, g2, b2) = rhs.into();
        (r1 + r2, g1 + g2, b1 + b2).into()
    }
}

impl Sub for LinRGB48Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.into();
        let (r2, g2, b2) = rhs.into();
        (r1 - r2, g1 - g2, b1 - b2).into()
    }
}

impl Mul<u16> for LinRGB48Color {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self::Output {
        let (r, g, b) = self.into();
        (r * rhs, g * rhs, b * rhs).into()
    }
}

impl Mul<LinRGB48Color> for u16 {
    type Output = LinRGB48Color;

    fn mul(self, rhs: LinRGB48Color) -> Self::Output {
        let (r, g, b) = rhs.into();
        (self * r, self * g, self * b).into()
    }
}

impl Div<u16> for LinRGB48Color {
    type Output = Self;

    fn div(self, rhs: u16) -> Self::Output {
        let (r, g, b) = self.into();
        (r / rhs, g / rhs, b / rhs).into()
    }
}

impl Div<LinRGB48Color> for u16 {
    type Output = LinRGB48Color;

    fn div(self, rhs: LinRGB48Color) -> Self::Output {
        let (r, g, b) = rhs.into();
        (self / r, self / g, self / b).into()
    }
}

impl fmt::Display for LinRGB48Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5}, {:>5}, {:>5}", self.r, self.g, self.b)
    }
}