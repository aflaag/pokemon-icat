use crate::*;

/// Representation of an RGB colour as 24-bit `0xRRGGBB` integer.
impl AsRGB for u32 {
    fn as_u32(&self) -> u32 { *self }
}

#[inline]
fn to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

impl AsRGB for (u8, u8, u8) {
    #[inline]
    fn as_u32(&self) -> u32 { to_u32(self.0, self.1, self.2) }
}

impl AsRGB for [u8; 3] {
    #[inline]
    fn as_u32(&self) -> u32 { to_u32(self[0], self[1], self[2]) }
}

impl<'a, T: AsRGB + ?Sized> AsRGB for &'a T {
    fn as_u32(&self) -> u32 { (*self).as_u32() }
}

#[cfg(feature = "rgb")]
trait Component: Copy {
    fn into_u8(self) -> u8;
}
#[cfg(feature = "rgb")]
impl Component for u8 {
    #[inline(always)]
    fn into_u8(self) -> u8 { self }
}
#[cfg(feature = "rgb")]
impl Component for u16 {
    #[inline(always)]
    fn into_u8(self) -> u8 { (self >> 8) as u8 }
}

#[cfg(feature = "rgb")]
impl<C: Component> AsRGB for rgb::RGB<C> {
    /// Returns representation of the sRGB colour as a 24-bit `0xRRGGBB`
    /// integer.
    ///
    /// This implementation is present only if `rgb` crate feature is enabled.
    /// Implementation is provided for `u8` and `u16` colour component types.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::{AsRGB, ansi256_from_rgb};
    ///
    /// assert_eq!(0x123456, rgb::RGB8::new(0x12, 0x34, 0x56).as_u32());
    ///
    /// assert_eq!( 16, ansi256_from_rgb(rgb::RGB8::new(  1,   1,   1)));
    /// assert_eq!( 16, ansi256_from_rgb(rgb::RGB8::new(  0,   1,   2)));
    /// assert_eq!( 67, ansi256_from_rgb(rgb::RGB8::new( 95, 135, 175)));
    /// assert_eq!(231, ansi256_from_rgb(rgb::RGB8::new(255, 255, 255)));
    ///
    /// assert_eq!(0x123456, rgb::RGB16::new(0x12ab, 0x34cd, 0x56ef).as_u32());
    ///
    /// assert_eq!( 16, ansi256_from_rgb(rgb::RGB16::new(  256,   511,   256)));
    /// assert_eq!( 16, ansi256_from_rgb(rgb::RGB16::new(  128,   256,   512)));
    /// assert_eq!( 67, ansi256_from_rgb(rgb::RGB16::new(24500, 34600, 44800)));
    /// assert_eq!(231, ansi256_from_rgb(rgb::RGB16::new(65535, 65535, 65535)));
    /// ```
    #[inline(always)]
    fn as_u32(&self) -> u32 {
        to_u32(self.r.into_u8(), self.g.into_u8(), self.b.into_u8())
    }
}

#[cfg(feature = "rgb")]
impl<C: Component> AsRGB for rgb::alt::Gray<C> {
    #[inline(always)]
    fn as_u32(&self) -> u32 { self.into_u8() as u32 * 0x010101 }

    /// Returns index of a colour in 256-colour ANSI palette approximating given
    /// shade of grey.
    ///
    /// This implementation is present only if `rgb` crate feature is enabled.
    /// Implementation is provided for `u8` and `u16` colour component types.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ansi256_from_rgb;
    ///
    /// assert_eq!(244, ansi256_from_rgb(rgb::alt::Gray::<u8>(128)));
    /// assert_eq!(244, ansi256_from_rgb(rgb::alt::Gray::<u16>(33023)));
    /// ```
    #[inline(always)]
    fn to_ansi256(&self) -> u8 { crate::ansi256_from_grey(self.into_u8()) }
}

#[cfg(feature = "rgb")]
impl<C: Component> AsRGB for rgb::alt::BGR<C> {
    /// Returns representation of the sRGB colour as a 24-bit `0xRRGGBB`
    /// integer.
    ///
    /// This implementation is present only if `rgb` crate feature is enabled.
    /// Implementation is provided for `u8` and `u16` colour component types.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::{AsRGB, ansi256_from_rgb};
    /// use rgb::alt::{BGR8, BGR16};
    ///
    /// assert_eq!(0x123456, BGR8 { b: 0x56, g: 0x34, r: 0x12 }.as_u32());
    ///
    /// assert_eq!( 16, ansi256_from_rgb(BGR8 { r:   1, g:   1, b:   1 }));
    /// assert_eq!( 16, ansi256_from_rgb(BGR8 { r:   0, g:   1, b:   2 }));
    /// assert_eq!( 67, ansi256_from_rgb(BGR8 { r:  95, g: 135, b: 175 }));
    /// assert_eq!(231, ansi256_from_rgb(BGR8 { r: 255, g: 255, b: 255 }));
    ///
    /// assert_eq!(0x123456, BGR16 { b: 0x56ef, g: 0x34cd, r: 0x12ab }.as_u32());
    ///
    /// assert_eq!( 16, ansi256_from_rgb(BGR16 { r:   256, g:   511, b:   256 }));
    /// assert_eq!( 16, ansi256_from_rgb(BGR16 { r:   128, g:   256, b:   512 }));
    /// assert_eq!( 67, ansi256_from_rgb(BGR16 { r: 24500, g: 34600, b: 44800 }));
    /// assert_eq!(231, ansi256_from_rgb(BGR16 { r: 65535, g: 65535, b: 65535 }));
    /// ```
    #[inline(always)]
    fn as_u32(&self) -> u32 {
        to_u32(self.r.into_u8(), self.g.into_u8(), self.b.into_u8())
    }
}

#[cfg(feature = "ansi_term")]
impl AsRGB for ansi_term::Colour {
    /// Returns sRGB colour corresponding to escape code represented by the
    /// object.
    ///
    /// Behaves slightly differently depending on the variant of the enum.
    /// - For named colour variants (`Black`, `Red` etc. up till `White`),
    ///   returns corresponding system colour with indexes going from 0 to 7.
    /// - For `Fixed` variant returns colour corresponding to specified index.
    ///   See [`rgb_from_ansi256`](`rgb_from_ansi256`).
    /// - For `RGB` variant converts it to 24-bit `0xRRGGBB` representation.
    ///
    /// This implementation is present only if `ansi_term` crate feature is
    /// enabled.
    #[inline]
    fn as_u32(&self) -> u32 {
        match *self {
            Self::Black => ansi256::ANSI_COLOURS[0],
            Self::Red => ansi256::ANSI_COLOURS[1],
            Self::Green => ansi256::ANSI_COLOURS[2],
            Self::Yellow => ansi256::ANSI_COLOURS[3],
            Self::Blue => ansi256::ANSI_COLOURS[4],
            Self::Purple => ansi256::ANSI_COLOURS[5],
            Self::Cyan => ansi256::ANSI_COLOURS[6],
            Self::White => ansi256::ANSI_COLOURS[7],
            Self::Fixed(idx) => ansi256::ANSI_COLOURS[usize::from(idx)],
            Self::RGB(r, g, b) => (r, g, b).as_u32(),
        }
    }

    /// Returns index of a colour in 256-colour ANSI palette approximating given
    /// sRGB colour.
    ///
    /// Behaves slightly differently depending on the variant of the enum.
    /// - For named colour variants (`Black`, `Red` etc. up till `White`),
    ///   returns index going from 0 to 7.
    /// - For `Fixed` variant simply returns index encoded in the variant.
    /// - Lastly, for `RGB` variant, approximates the colour and returns index
    ///   of closest colour in 256-colour palette.
    ///
    /// This implementation is present only if `ansi_term` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::AsRGB;
    ///
    /// assert_eq!(  0, ansi_term::Colour::Black.to_ansi256());
    /// assert_eq!(  7, ansi_term::Colour::White.to_ansi256());
    /// assert_eq!( 42, ansi_term::Colour::Fixed(42).to_ansi256());
    /// assert_eq!( 16, ansi_term::Colour::RGB(  0,   0,   0).to_ansi256());
    /// assert_eq!( 16, ansi_term::Colour::RGB(  1,   1,   1).to_ansi256());
    /// assert_eq!( 16, ansi_term::Colour::RGB(  0,   1,   2).to_ansi256());
    /// assert_eq!( 67, ansi_term::Colour::RGB( 95, 135, 175).to_ansi256());
    /// assert_eq!(231, ansi_term::Colour::RGB(255, 255, 255).to_ansi256());
    /// ```
    #[inline]
    fn to_ansi256(&self) -> u8 {
        match *self {
            Self::Black => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::Blue => 4,
            Self::Purple => 5,
            Self::Cyan => 6,
            Self::White => 7,
            Self::Fixed(idx) => idx,
            Self::RGB(r, g, b) => (r, g, b).to_ansi256(),
        }
    }
}

#[cfg(feature = "ansi_term")]
impl super::ColourExt for ansi_term::Colour {
    /// Constructs a `Fixed` colour which approximates given sRGB colour.
    ///
    /// This implementation is present only if `ansi_term` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use ansi_term::Colour;
    ///
    /// assert_eq!(Colour::Fixed( 16), Colour::approx_rgb(  0,   0,   0));
    /// assert_eq!(Colour::Fixed( 16), Colour::approx_rgb(  0,   1,   2));
    /// assert_eq!(Colour::Fixed( 67), Colour::approx_rgb( 95, 135, 175));
    /// assert_eq!(Colour::Fixed(231), Colour::approx_rgb(255, 255, 255));
    /// ```
    #[inline]
    fn approx_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Fixed(ansi256_from_rgb((r, g, b)))
    }

    /// Converts the colour into 256-colour-compatible format.
    ///
    /// If the colour represents an RGB colour, converts it into a `Fixed`
    /// variant using [`ansi256_from_rgb`] function.  Otherwise, returns the
    /// colour unchanged.
    ///
    /// This implementation is present only if `ansi_term` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use ansi_term::Colour;
    ///
    /// assert_eq!(Colour::Red,        Colour::Red.to_256());
    /// assert_eq!(Colour::Fixed( 11), Colour::Fixed(11).to_256());
    /// assert_eq!(Colour::Fixed( 16), Colour::RGB(  0,   0,   0).to_256());
    /// assert_eq!(Colour::Fixed( 16), Colour::RGB(  0,   1,   2).to_256());
    /// assert_eq!(Colour::Fixed( 67), Colour::RGB( 95, 135, 175).to_256());
    /// assert_eq!(Colour::Fixed(231), Colour::RGB(255, 255, 255).to_256());
    /// ```
    #[inline]
    fn to_256(&self) -> Self {
        match *self {
            Self::RGB(r, g, b) => Self::approx_rgb(r, g, b),
            colour => colour,
        }
    }

    /// Converts the colour into sRGB.
    ///
    /// Named colours (`Black`, `Red` etc. through `White`) are treated like
    /// `Fixed` colours with indexes 0 through 7.  `Fixed` colours are converted
    /// into sRGB using [`rgb_from_ansi256`] function.  `RGB` colours are
    /// returned unchanged.
    ///
    /// This implementation is present only if `ansi_term` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use ansi_term::Colour;
    ///
    /// assert_eq!((  0,   0,   0), Colour::Fixed( 16).to_rgb());
    /// assert_eq!(( 95, 135, 175), Colour::Fixed( 67).to_rgb());
    /// assert_eq!((255, 255, 255), Colour::Fixed(231).to_rgb());
    /// assert_eq!((238, 238, 238), Colour::Fixed(255).to_rgb());
    /// assert_eq!(( 42,  24,   0), Colour::RGB(42, 24, 0).to_rgb());
    /// ```
    #[inline]
    fn to_rgb(&self) -> (u8, u8, u8) {
        let idx = match *self {
            Self::Black => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::Blue => 4,
            Self::Purple => 5,
            Self::Cyan => 6,
            Self::White => 7,
            Self::Fixed(idx) => idx,
            Self::RGB(r, g, b) => return (r, g, b),
        };
        rgb_from_ansi256(idx)
    }
}

#[cfg(feature = "termcolor")]
impl AsRGB for termcolor::Color {
    /// Returns sRGB colour corresponding to escape code represented by
    /// [`termcolor::Color`].
    ///
    /// Behaves slightly differently depending on the variant of the enum.
    /// - For named colour variants (`Black`, `Red` etc. up till `White`),
    ///   returns corresponding system colour with indexes going from 0 to 7.
    /// - For `Ansi256` variant returns colour corresponding to specified index.
    ///   See [`rgb_from_ansi256`](`rgb_from_ansi256`).
    /// - For `Rgb` variant converts it to 24-bit `0xRRGGBB` representation.
    ///
    /// This implementation is present only if `termcolor` crate feature is
    /// enabled.
    #[inline]
    fn as_u32(&self) -> u32 {
        match *self {
            Self::Black => ansi256::ANSI_COLOURS[0],
            Self::Blue => ansi256::ANSI_COLOURS[4],
            Self::Green => ansi256::ANSI_COLOURS[2],
            Self::Red => ansi256::ANSI_COLOURS[1],
            Self::Cyan => ansi256::ANSI_COLOURS[6],
            Self::Magenta => ansi256::ANSI_COLOURS[5],
            Self::Yellow => ansi256::ANSI_COLOURS[3],
            Self::White => ansi256::ANSI_COLOURS[7],
            Self::Ansi256(idx) => ansi256::ANSI_COLOURS[usize::from(idx)],
            Self::Rgb(r, g, b) => (r, g, b).as_u32(),
            _ => unreachable!(),
        }
    }

    /// Returns index of a colour in 256-colour ANSI palette approximating given
    /// sRGB colour.
    ///
    /// Behaves slightly differently depending on the variant of the enum.
    /// - For named colour variants (`Black`, `Red` etc. up till `White`),
    ///   returns index going from 0 to 7.
    /// - For `Ansi256` variant simply returns index encoded in the variant.
    /// - Lastly, for `Rgb` variant, approximates the colour and returns index
    ///   of closest colour in 256-colour palette.
    ///
    /// This implementation is present only if `termcolor` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::AsRGB;
    ///
    /// assert_eq!(  0, termcolor::Color::Black.to_ansi256());
    /// assert_eq!(  7, termcolor::Color::White.to_ansi256());
    /// assert_eq!( 42, termcolor::Color::Ansi256(42).to_ansi256());
    /// assert_eq!( 16, termcolor::Color::Rgb(  0,   0,   0).to_ansi256());
    /// assert_eq!( 16, termcolor::Color::Rgb(  1,   1,   1).to_ansi256());
    /// assert_eq!( 16, termcolor::Color::Rgb(  0,   1,   2).to_ansi256());
    /// assert_eq!( 67, termcolor::Color::Rgb( 95, 135, 175).to_ansi256());
    /// assert_eq!(231, termcolor::Color::Rgb(255, 255, 255).to_ansi256());
    /// ```
    #[inline]
    fn to_ansi256(&self) -> u8 {
        match *self {
            Self::Black => 0,
            Self::Blue => 4,
            Self::Green => 2,
            Self::Red => 1,
            Self::Cyan => 6,
            Self::Magenta => 5,
            Self::Yellow => 3,
            Self::White => 7,
            Self::Ansi256(idx) => idx,
            Self::Rgb(r, g, b) => (r, g, b).to_ansi256(),
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "termcolor")]
impl super::ColourExt for termcolor::Color {
    /// Constructs a `Ansi256` colour which approximates given sRGB colour.
    ///
    /// This implementation is present only if `termcolor` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use termcolor::Color;
    ///
    /// assert_eq!(Color::Ansi256( 16), Color::approx_rgb(  0,   0,   0));
    /// assert_eq!(Color::Ansi256( 16), Color::approx_rgb(  0,   1,   2));
    /// assert_eq!(Color::Ansi256( 67), Color::approx_rgb( 95, 135, 175));
    /// assert_eq!(Color::Ansi256(231), Color::approx_rgb(255, 255, 255));
    /// ```
    #[inline]
    fn approx_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Ansi256(ansi256_from_rgb((r, g, b)))
    }

    /// Converts the colour into 256-colour-compatible format.
    ///
    /// If the colour represents an RGB colour, converts it into an `Ansi256`
    /// variant using [`ansi256_from_rgb`] function.  Otherwise, returns the
    /// colour unchanged.
    ///
    /// This implementation is present only if `termcolor` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use termcolor::Color;
    ///
    /// assert_eq!(Color::Red,          Color::Red.to_256());
    /// assert_eq!(Color::Ansi256( 11), Color::Ansi256(11).to_256());
    /// assert_eq!(Color::Ansi256( 16), Color::Rgb(  0,   0,   0).to_256());
    /// assert_eq!(Color::Ansi256( 16), Color::Rgb(  0,   1,   2).to_256());
    /// assert_eq!(Color::Ansi256( 67), Color::Rgb( 95, 135, 175).to_256());
    /// assert_eq!(Color::Ansi256(231), Color::Rgb(255, 255, 255).to_256());
    /// ```
    #[inline]
    fn to_256(&self) -> Self {
        match *self {
            Self::Rgb(r, g, b) => Self::approx_rgb(r, g, b),
            colour => colour,
        }
    }

    /// Converts the colour into sRGB.
    ///
    /// Named colours (`Black`, `Red` etc. through `White`) are treated like
    /// `Ansi256` colours with indexes 0 through 7.  `Ansi256` colours are
    /// converted into sRGB using [`rgb_from_ansi256`] function.  `Rgb` colours
    /// are returned unchanged.
    ///
    /// This implementation is present only if `termcolor` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use termcolor::Color;
    ///
    /// assert_eq!((  0,   0,   0), Color::Ansi256( 16).to_rgb());
    /// assert_eq!(( 95, 135, 175), Color::Ansi256( 67).to_rgb());
    /// assert_eq!((255, 255, 255), Color::Ansi256(231).to_rgb());
    /// assert_eq!((238, 238, 238), Color::Ansi256(255).to_rgb());
    /// assert_eq!(( 42,  24,   0), Color::Rgb(42, 24, 0).to_rgb());
    /// ```
    #[inline]
    fn to_rgb(&self) -> (u8, u8, u8) {
        let idx = match *self {
            Self::Black => 0,
            Self::Blue => 4,
            Self::Green => 2,
            Self::Red => 1,
            Self::Cyan => 6,
            Self::Magenta => 5,
            Self::Yellow => 3,
            Self::White => 7,
            Self::Ansi256(idx) => idx,
            Self::Rgb(r, g, b) => return (r, g, b),
            _ => unreachable!(),
        };
        rgb_from_ansi256(idx)
    }
}

#[cfg(feature = "anstyle")]
impl AsRGB for anstyle::RgbColor {
    /// Returns representation of the sRGB colour as a 24-bit `0xRRGGBB`
    /// integer.
    ///
    /// This implementation is present only if `anstyle` crate feature is
    /// enabled.
    #[inline(always)]
    fn as_u32(&self) -> u32 { to_u32(self.0, self.1, self.2) }
}

#[cfg(feature = "anstyle")]
impl ColourExt for anstyle::Ansi256Color {
    /// Constructs an colour which best approximates given sRGB colour.
    ///
    /// This implementation is present only if `anstyle` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use anstyle::Ansi256Color;
    ///
    /// assert_eq!(Ansi256Color( 16), Ansi256Color::approx_rgb(  0,   0,   0));
    /// assert_eq!(Ansi256Color( 16), Ansi256Color::approx_rgb(  0,   1,   2));
    /// assert_eq!(Ansi256Color( 67), Ansi256Color::approx_rgb( 95, 135, 175));
    /// assert_eq!(Ansi256Color(231), Ansi256Color::approx_rgb(255, 255, 255));
    /// ```
    #[inline(always)]
    fn approx_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(ansi256_from_rgb((r, g, b)))
    }

    /// Returns `self`.
    ///
    /// This implementation is present only if `anstyle` crate feature is
    /// enabled.
    #[inline(always)]
    fn to_256(&self) -> Self { *self }

    /// Converts the colour into sRGB.
    ///
    /// This implementation is present only if `anstyle` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use anstyle::Ansi256Color;
    ///
    /// assert_eq!((  0,   0,   0), Ansi256Color( 16).to_rgb());
    /// assert_eq!(( 95, 135, 175), Ansi256Color( 67).to_rgb());
    /// assert_eq!((255, 255, 255), Ansi256Color(231).to_rgb());
    /// assert_eq!((238, 238, 238), Ansi256Color(255).to_rgb());
    /// ```
    #[inline(always)]
    fn to_rgb(&self) -> (u8, u8, u8) { rgb_from_ansi256(self.0) }
}

#[cfg(feature = "anstyle")]
impl ColourExt for anstyle::Color {
    /// Constructs an ANSI 256 colour which best approximates given sRGB colour.
    ///
    /// This implementation is present only if `anstyle` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use anstyle::{Ansi256Color, Color};
    ///
    /// assert_eq!(Color::Ansi256(Ansi256Color( 16)),
    ///            Color::approx_rgb(  0,   0,   0));
    /// assert_eq!(Color::Ansi256(Ansi256Color( 16)),
    ///            Color::approx_rgb(  0,   1,   2));
    /// assert_eq!(Color::Ansi256(Ansi256Color( 67)),
    ///            Color::approx_rgb( 95, 135, 175));
    /// assert_eq!(Color::Ansi256(Ansi256Color(231)),
    ///            Color::approx_rgb(255, 255, 255));
    /// ```
    #[inline]
    fn approx_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Ansi256(anstyle::Ansi256Color::approx_rgb(r, g, b))
    }

    /// Converts the colour into 256-colour-compatible format.
    ///
    /// If the colour represents an RGB colour, converts it into an `Ansi256`
    /// variant using [`ansi256_from_rgb`] function.  Otherwise, returns the
    /// colour unchanged.
    ///
    /// This implementation is present only if `anstyle` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ansi_colours::ColourExt;
    /// use anstyle::{Ansi256Color, AnsiColor, Color, RgbColor};
    ///
    /// assert_eq!(Color::Ansi(AnsiColor::Red),
    ///            Color::Ansi(AnsiColor::Red).to_256());
    /// assert_eq!(Color::Ansi256(Ansi256Color( 11)),
    ///            Color::Ansi256(Ansi256Color( 11)).to_256());
    /// assert_eq!(Color::Ansi256(Ansi256Color( 16)),
    ///            Color::Rgb(RgbColor(  0,   0,   0)).to_256());
    /// assert_eq!(Color::Ansi256(Ansi256Color( 16)),
    ///            Color::Rgb(RgbColor(  0,   1,   2)).to_256());
    /// assert_eq!(Color::Ansi256(Ansi256Color( 67)),
    ///            Color::Rgb(RgbColor( 95, 135, 175)).to_256());
    /// assert_eq!(Color::Ansi256(Ansi256Color(231)),
    ///            Color::Rgb(RgbColor(255, 255, 255)).to_256());
    /// ```
    #[inline]
    fn to_256(&self) -> Self {
        match *self {
            Self::Rgb(anstyle::RgbColor(r, g, b)) => Self::approx_rgb(r, g, b),
            colour => colour,
        }
    }

    /// Converts the colour into sRGB.
    ///
    /// `AnsiColour` and `Ansi256Color` colour variants are converted into sRGB
    /// using [`rgb_from_ansi256`] function.  `Rgb` colours are returned
    /// unchanged.
    ///
    /// This implementation is present only if `anstyle` crate feature is
    /// enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ansi_colours::ColourExt;
    /// use anstyle::{Ansi256Color, AnsiColor, Color, RgbColor};
    ///
    /// assert_eq!((  0,   0,   0), Color::Ansi256(Ansi256Color( 16)).to_rgb());
    /// assert_eq!(( 95, 135, 175), Color::Ansi256(Ansi256Color( 67)).to_rgb());
    /// assert_eq!((255, 255, 255), Color::Ansi256(Ansi256Color(231)).to_rgb());
    /// assert_eq!((238, 238, 238), Color::Ansi256(Ansi256Color(255)).to_rgb());
    /// assert_eq!(( 42,  24,   0), Color::Rgb(RgbColor(42, 24, 0)).to_rgb());
    /// ```
    #[inline(always)]
    fn to_rgb(&self) -> (u8, u8, u8) {
        rgb_from_ansi256(match *self {
            Self::Ansi(colour) => colour as u8,
            Self::Ansi256(colour) => colour.0,
            Self::Rgb(anstyle::RgbColor(r, g, b)) => return (r, g, b),
        })
    }
}
