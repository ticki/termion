//! Colors.

use std::fmt;

/// A terminal color.
pub trait Color {
    /// Write the foreground version of this color.
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result;
    /// Write the background version of this color.
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

macro_rules! derive_color {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        #[derive(Copy, Clone)]
        pub struct $name;

        impl Color for $name {
            #[inline]
            fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, csi!("38;5;", $value, "m"))
            }

            #[inline]
            fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, csi!("48;5;", $value, "m"))
            }
        }
    };
}

derive_color!("Black.", Black, "0");
derive_color!("Red.", Red, "1");
derive_color!("Green.", Green, "2");
derive_color!("Yellow.", Yellow, "3");
derive_color!("Blue.", Blue, "4");
derive_color!("Magenta.", Magenta, "5");
derive_color!("Cyan.", Cyan, "6");
derive_color!("White.", White, "7");
derive_color!("High-intensity light black.", LightBlack, "8");
derive_color!("High-intensity light red.", LightRed, "9");
derive_color!("High-intensity light green.", LightGreen, "10");
derive_color!("High-intensity light yellow.", LightYellow, "11");
derive_color!("High-intensity light blue.", LightBlue, "12");
derive_color!("High-intensity light magenta.", LightMagenta, "13");
derive_color!("High-intensity light cyan.", LightCyan, "14");
derive_color!("High-intensity light white.", LightWhite, "15");

/// An arbitrary ANSI color value.
#[derive(Clone, Copy)]
pub struct AnsiValue(pub u8);

impl AnsiValue {
    /// 216-color (r, g, b ≤ 5) RGB.
    pub fn rgb(r: u8, g: u8, b: u8) -> AnsiValue {
        debug_assert!(r <= 5, "Red color fragment (r = {}) is out of bound. Make sure r ≤ 5.", r);
        debug_assert!(g <= 5, "Green color fragment (g = {}) is out of bound. Make sure g ≤ 5.", g);
        debug_assert!(b <= 5, "Blue color fragment (b = {}) is out of bound. Make sure b ≤ 5.", b);

        AnsiValue(16 + 36 * r + 6 * g + b)
    }

    /// Grayscale color.
    ///
    /// There are 24 shades of gray.
    pub fn grayscale(shade: u8) -> AnsiValue {
        // Unfortunately, there are a little less than fifty shades.
        debug_assert!(shade < 24, "Grayscale out of bound (shade = {}). There are only 24 shades of \
                      gray.", shade);

        AnsiValue(0xE8 + shade)
    }
}

impl Color for AnsiValue {
    #[inline]
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("38;5;{}m"), self.0)
    }

    #[inline]
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("48;5;{}m"), self.0)
    }
}

/// A truecolor RGB.
pub struct Rgb(pub u8, pub u8, pub u8);

impl Color for Rgb {
    #[inline]
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("38;2;{};{};{}m"), self.0, self.1, self.2)
    }

    #[inline]
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("48;2;{};{};{}m"), self.0, self.1, self.2)
    }
}

/// Reset colors to defaults.
pub struct Reset;

impl Color for Reset {
    #[inline]
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("39m"))
    }

    #[inline]
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("49m"))
    }
}

/// A foreground color.
pub struct Fg<C: Color>(pub C);

impl<C: Color> fmt::Display for Fg<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.write_fg(f)
    }
}

/// A background color.
pub struct Bg<C: Color>(pub C);

impl<C: Color> fmt::Display for Bg<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.write_bg(f)
    }
}
