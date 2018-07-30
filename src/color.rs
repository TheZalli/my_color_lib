use std::str;
use std::fmt;

const GAMMA: f32 = 2.4;

#[inline] fn gamma_encode(linear: f32) -> f32 { linear.powf(1.0/GAMMA) }
#[inline] fn gamma_decode(encoded: f32) -> f32 { encoded.powf(GAMMA) }

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// The basic colors of the rainbow
pub enum BaseColor {
    Black,
    Grey,
    White,
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

impl fmt::Display for BaseColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BaseColor::*;

        write!(f, "{}",
            match *self {
                Black   => "black",
                Grey    => "grey",
                White   => "white",
                Red     => "red",
                Yellow  => "yellow",
                Green   => "green",
                Cyan    => "cyan",
                Blue    => "blue",
                Magenta => "magenta",
            }
        )
    }
}

pub trait Color {
    /// Returns this color in the normalized sRGB color space
    fn srgb(&self) -> SRGBColor;

    /// Returns this color in the 24-bit sRGB color space
    fn srgb24(&self) -> SRGB24Color { self.srgb().srgb24() }

    /// Return the normalised RGB representation in the linear color space
    fn lin_rgb(&self) -> LinRGBColor { self.srgb().lin_rgb() }

    /// Return the 24-bit RGB representation in the linear color space
    fn lin_rgb24(&self) -> LinRGB24Color { self.lin_rgb().lin_rgb24() }

    /// Return the HSV representation
    fn hsv(&self) -> HSVColor { self.srgb().hsv() }

    /// Returns the relative luminance of this color between 0 and 1.
    ///
    /// Tells the whiteness of the color as perceived by humans.
    /// Values nearer 0 are darker, and values nearer 1 are lighter.
    fn relative_luminance(&self) -> f32 {
        let (r, g, b) = self.lin_rgb().to_tuple();
        0.2126*r + 0.7152*g + 0.0722*b
    }

    /// Categorize this color's most prominent shades
    fn shades(&self) ->  Vec<(BaseColor, f32)> {
        use self::BaseColor::*;

        const COLOR_HUES: [(f32, BaseColor); 5] =
            [(60.0, Yellow),
             (120.0, Green),
             (180.0, Cyan),
             (240.0, Blue),
             (300.0, Magenta)];

        // all of these borders have been picked by what looks nice
        // they could be improved

        // how many degrees from the main hue can a shade be
        const HUE_MARGIN: f32 = 60.0 * 0.75;

        // relative luminance under this value is considered to be just black
        const BLACK_CUTOFF_LUMINANCE: f32 = 0.005;

        // saturation under this value is considered to be just greyscale without any color
        const GREYSCALE_SATURATION: f32 = 0.05;

        // borders for the greyscale shades
        const WHITE_SATURATION: f32 = 0.35;
        const WHITE_LUMINANCE: f32 = 0.40;

        const GREY_SATURATION: f32 = 0.45;
        const GREY_LUMINANCE_MAX: f32 = 0.80;
        const GREY_LUMINANCE_MIN: f32 = 0.03;

        const BLACK_LUMINANCE: f32 = 0.045;

        let mut shades = Vec::with_capacity(3);

        let (h, s, _v) = self.hsv().to_tuple();
        let lum = self.relative_luminance();

        if lum < BLACK_CUTOFF_LUMINANCE {
            return vec![(Black, 1.0)];
        }

        let mut sum = 0.0;

        if s > GREYSCALE_SATURATION {
            // red is a special case
            if h >= 360.0 - HUE_MARGIN || h <= 0.0 + HUE_MARGIN {
                let amount = 1.0 -
                    if h <= 0.0 + HUE_MARGIN {
                        h
                    } else {
                        h - 360.0
                    } / HUE_MARGIN;

                sum += amount;
                shades.push((Red, amount));
            }
            for (hue, color) in COLOR_HUES.iter() {
                let dist = (h - hue).abs();
                if dist <= HUE_MARGIN {
                    let amount = 1.0 - dist / HUE_MARGIN;
                    sum += amount;
                    shades.push((*color, amount));
                }
            }
        }

        if lum <= BLACK_LUMINANCE {
            sum += 1.0;
            shades.push((Black, 1.0));
        } else if lum >= WHITE_LUMINANCE && s <= WHITE_SATURATION {
            //let amount = 1.0 - (WHITE_SATURATION - s) / WHITE_SATURATION;
            sum += 1.0;
            shades.push((White, 1.0));
        }

        if s <= GREY_SATURATION && lum <= GREY_LUMINANCE_MAX && lum >= GREY_LUMINANCE_MIN {
            //let amount = 1.0 - (GREY_SATURATION - s) / GREY_SATURATION;
            sum += 1.0;
            shades.push((Grey, 1.0));
        }
        // sort and normalize
        shades.sort_unstable_by(
            |(_, amount), (_, amount2)| amount2.partial_cmp(amount).unwrap()
        );

        return shades.iter_mut().map(|(color, amount)| (*color, *amount/sum)).collect();
    }

    /// Returns the `text` with this color as it's background color using ANSI escapes.
    fn ansi_bgcolor(&self, text: &str) -> String {
        const CSI: &str = "\u{1B}[";
        let (r, g, b) = self.srgb24().to_tuple();

        // color the text as black or white depending on the bg:s lightness
        let fg =
            if self.relative_luminance() < gamma_decode(0.5) {
                format!("{}38;2;255;255;255m", CSI)
            } else {
                format!("{}38;2;;;m", CSI)
            };

        fg + &format!("{}48;2;{};{};{}m{}{0}0m", CSI, r, g, b, text)
    }
}

impl Color for BaseColor {
    fn srgb(&self) -> SRGBColor { self.srgb24().srgb() }

    fn srgb24(&self) -> SRGB24Color {
        use self::BaseColor::*;

        let f = &SRGB24Color::new;
        match self {
            Black   => f(  0,   0,   0),
            Grey    => f(128, 128, 128),
            White   => f(255, 255, 255),
            Red     => f(255,   0,   0),
            Yellow  => f(255, 255,   0),
            Green   => f(  0, 255,   0),
            Cyan    => f(  0, 255, 255),
            Blue    => f(  0,   0, 255),
            Magenta => f(255,   0, 255),
        }
    }

    fn hsv(&self) -> HSVColor {
        use self::BaseColor::*;

        let f = &HSVColor::new;
        match self {
            Black   => f(  0.0, 0.0, 0.0),
            Grey    => f(  0.0, 0.0, 0.5),
            White   => f(  0.0, 0.0, 1.0),
            Red     => f(  0.0, 1.0, 1.0),
            Yellow  => f( 60.0, 1.0, 1.0),
            Green   => f(120.0, 1.0, 1.0),
            Cyan    => f(180.0, 1.0, 1.0),
            Blue    => f(240.0, 1.0, 1.0),
            Magenta => f(300.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
/// An sRGB color with channels normalized between 0 and 1.
pub struct SRGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl SRGBColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self { SRGBColor { r, g, b } }
    pub fn to_tuple(&self) -> (f32, f32, f32) { (self.r, self.g, self.b) }
}

impl Color for SRGBColor {
    fn srgb(&self) -> SRGBColor { *self }

    fn srgb24(&self) -> SRGB24Color {
        let (r, g, b) = self.to_tuple();
        SRGB24Color::new((255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8)
    }

    fn lin_rgb(&self) -> LinRGBColor {
        const SRGB_INV_CUTOFF: f32 = 0.04045;

        let decode = |encoded|
            if encoded <= SRGB_INV_CUTOFF {
                encoded / 12.92
            } else {
                gamma_decode((encoded + 0.055)/1.055)
            };

        let (r, g, b) = self.to_tuple();
        LinRGBColor::new(decode(r), decode(g), decode(b))
    }

    fn hsv(&self) -> HSVColor {
        let (r, g, b) = self.to_tuple();

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let value = max;
        let saturation = if max == 0.0 { 0.0 } else { delta / max };
        let hue = 60.0 *
            if delta == 0.0 {
                0.0
            } else if max == r {
                ((g - b) / delta) % 6.0
            } else if max == g {
                (b - r) / delta + 2.0
            } else { // max == b
                (r - g) / delta + 4.0
            };

        HSVColor::new(hue, saturation, value)
    }
}

impl fmt::Display for SRGBColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}%, {:>5.1}%, {:>5.1}%", self.r * 100.0, self.g * 100.0, self.b * 100.0)
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SRGB24Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl SRGB24Color {
    /// Create a new RGB color.
    pub fn new(r: u8, g: u8, b: u8) -> Self { SRGB24Color { r, g, b } }

    /// Destructure self into a tuple
    pub fn to_tuple(&self) -> (u8, u8, u8) { (self.r, self.g, self.b) }

    /// Create `SRGB24Color` from a hexcode.
    ///
    /// # Safety
    /// If `hex_str` is not a valid utf-8 string then this function will result in undefined
    /// behaviour.
    ///
    /// If `hex_str` doesn't consist only of the characters `[0-9a-fA-F]` then this function will
    /// result in a panic.
    pub unsafe fn from_hex_unchecked(hex_str: Box<str>) -> Self {
        let f = |h1: u8, h2: u8|
            u8::from_str_radix(str::from_utf8_unchecked(&[h1, h2]), 16).unwrap();

        let mut hex_str = hex_str;
        let h = hex_str.as_bytes_mut();
        h.make_ascii_lowercase();

        SRGB24Color::new(f(h[0], h[1]), f(h[2], h[3]), f(h[4], h[5]))
    }
}

impl Color for SRGB24Color {
    fn srgb(&self) -> SRGBColor {
        let (r, g, b) = self.to_tuple();
        SRGBColor::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    fn srgb24(&self) -> SRGB24Color { *self }
}

impl fmt::Display for SRGB24Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3}, {:>3}, {:>3}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
/// An RGB color with channels normalized between 0 and 1 in the linear space.
pub struct LinRGBColor {
    r: f32,
    g: f32,
    b: f32
}

impl LinRGBColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self { LinRGBColor { r, g, b } }
    pub fn to_tuple(&self) -> (f32, f32, f32) { (self.r, self.g, self.b) }
}

impl Color for LinRGBColor {
    fn srgb(&self) -> SRGBColor {
        const SRGB_CUTOFF: f32 = 0.0031308;

        let encode = |linear|
            if linear <= SRGB_CUTOFF {
                linear * 12.92
            } else {
                gamma_encode(linear) * 1.055 - 0.055
            };

        let (r, g, b) = self.to_tuple();
        SRGBColor::new(encode(r), encode(g), encode(b))
    }

    fn lin_rgb(&self) -> LinRGBColor { *self }

    fn lin_rgb24(&self) -> LinRGB24Color {
        let (r, g, b) = self.to_tuple();
        LinRGB24Color::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}

impl fmt::Display for LinRGBColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}%, {:>5.1}%, {:>5.1}%", self.r * 100.0, self.g * 100.0, self.b * 100.0)
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// A 24-bit color with red, green and blue channels in the linear color space.
pub struct LinRGB24Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl LinRGB24Color {
    /// Create a new sRGB color.
    pub fn new(r: u8, g: u8, b: u8) -> Self { LinRGB24Color { r, g, b } }

    /// Destructure self into a tuple
    pub fn to_tuple(&self) -> (u8, u8, u8) { (self.r, self.g, self.b) }
}

impl Color for LinRGB24Color {
    fn srgb(&self) -> SRGBColor { self.lin_rgb().srgb() }

    fn lin_rgb(&self) -> LinRGBColor {
        let (r, g, b) = self.to_tuple();
        LinRGBColor::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    fn lin_rgb24(&self) -> LinRGB24Color { *self }
}

impl fmt::Display for LinRGB24Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3}, {:>3}, {:>3}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct HSVColor {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    _priv: ()
}

impl HSVColor {
    /// Create a new HSV value.
    ///
    /// Hue is given in degrees and it is wrapped between [0, 360).
    /// Saturation and value are given as a percentage between \[0, 1\].
    ///
    /// # Panic
    /// If saturation and value are not between 0.0 and 1.0, this function will panic.
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        if s < 0.0 || s > 1.0 {
            panic!("Invalid HSV saturation: {}", s);
        }
        if v < 0.0 || v > 1.0 {
            panic!("Invalid HSV value: {}", v);
        }

        let mut h = h % 360.0;
        if h < 0.0 {
            h = h + 360.0;
        }
        HSVColor { h, s, v, _priv: () }
    }

    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.h, self.s, self.v)
    }
}

impl Color for HSVColor {
    fn srgb(&self) -> SRGBColor {
        let (h, s, v) = self.to_tuple();
        let h = h / 60.0;

        // largest, second largest and the smallest component
        let c = s * v;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let min = v - c;

        let (r, g, b) =
            match h as u8 {
                0   => (  c,   x, 0.0),
                1   => (  x,   c, 0.0),
                2   => (0.0,   c,   x),
                3   => (0.0,   x,   c),
                4   => (  x, 0.0,   c),
                5|6 => (  c, 0.0,   x),
                _   => panic!("Invalid hue value: {}", self.h)
            };

        SRGBColor::new(r+min, g+min, b+min)
    }

    fn hsv(&self) -> HSVColor { *self }
}

impl fmt::Display for HSVColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}°, {:>5.1}%, {:>5.1}%", self.h, self.s * 100.0, self.v * 100.0)
    }
}
