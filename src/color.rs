/// A terminal color.
pub trait Color {
    /// Get an escape code corresponding to the color value.
    fn to_escape_code(self, is_background: bool) -> String;
}

macro_rules! derive_color {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        pub struct $name;

        impl Color for $name {
            #[inline]
            fn to_escape_code(self, is_background: bool) -> String {
                ansi_escape_code($value, is_background)
            }
        }
    };
}

/// Compute the escape code for an ANSI coded color.
#[inline]
fn ansi_escape_code(number: u8, is_background: bool) -> String {
    let bg_code = if is_background { 48 } else { 38 };
    format!("{};5;{}m", bg_code, number)
}

derive_color!("Black.", Black, 0x0);
derive_color!("Red.", Red, 0x1);
derive_color!("Green.", Green, 0x2);
derive_color!("Yellow.", Yellow, 0x3);
derive_color!("Blue.", Blue, 0x4);
derive_color!("Magenta.", Magenta, 0x5);
derive_color!("Cyan.", Cyan, 0x6);
derive_color!("White.", White, 0x7);
derive_color!("High-intensity light black.", LightBlack, 0x8);
derive_color!("High-intensity light red.", LightRed, 0x9);
derive_color!("High-intensity light green.", LightGreen, 0xA);
derive_color!("High-intensity light yellow.", LightYellow, 0xB);
derive_color!("High-intensity light blue.", LightBlue, 0xC);
derive_color!("High-intensity light magenta.", LightMagenta, 0xD);
derive_color!("High-intensity light cyan.", LightCyan, 0xE);
derive_color!("High-intensity light white.", LightWhite, 0xF);

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

/// An arbitrary ANSI color value.
#[derive(Clone, Copy)]
pub struct AnsiValue(pub u8);

impl Color for AnsiValue {
    #[inline]
    fn to_escape_code(self, is_background: bool) -> String {
        ansi_escape_code(self.0, is_background)
    }
}

/// A true 24 bit color value.
#[derive(Clone, Copy)]
pub struct TrueColorValue{
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
}

impl Color for TrueColorValue {
    #[inline]
    fn to_escape_code(self, is_background: bool) -> String {
        let bg_code = if is_background { 48 } else { 38 };
        format!("{};2;{};{};{}m", bg_code, self.r, self.g, self.b)
    }
}

/// 24 bit true RGB.
pub fn true_rgb(r: u8, g: u8, b: u8) -> TrueColorValue {
    TrueColorValue{r: r, g: g, b: b}
}

/// A color palette.
///
/// This should generally only be used when the color is runtime determined. Otherwise, use the
/// color types, which resolves the value at compile time.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Palette {
    /// Black.
    Black,
    /// Red.
    Red,
    /// Green.
    Green,
    /// Yellow.
    Yellow,
    /// Blue.
    Blue,
    /// Megenta.
    Magenta,
    /// Cyan.
    Cyan,
    /// White.
    White,
    /// High-intensity black.
    LightBlack,
    /// High-intensity red.
    LightRed,
    /// High-intensity green.
    LightGreen,
    /// High-intensity yellow.
    LightYellow,
    /// High-intensity blue.
    LightBlue,
    /// High-intensity magenta.
    LightMagenta,
    /// High-intensity cyan.
    LightCyan,
    /// High-intensity white.
    LightWhite,
    /// 216-color (r, g, b ≤ 5) RGB.
    Rgb(u8, u8, u8),
    /// 24 bit true RGB color.
    TrueRgb(u8, u8, u8),
    /// Grayscale (max value: 24).
    Grayscale(u8),
}

impl Color for Palette {
    fn to_escape_code(self, is_background: bool) -> String {
        match self {
            Palette::Rgb(r, g, b) => rgb(r, g, b).to_escape_code(is_background),
            Palette::TrueRgb(r, g, b) => true_rgb(r, g, b).to_escape_code(is_background),
            Palette::Grayscale(shade) => grayscale(shade).to_escape_code(is_background),
            Palette::Black => Black.to_escape_code(is_background),
            Palette::Red => Red.to_escape_code(is_background),
            Palette::Green => Green.to_escape_code(is_background),
            Palette::Yellow => Yellow.to_escape_code(is_background),
            Palette::Blue => Blue.to_escape_code(is_background),
            Palette::Magenta => Magenta.to_escape_code(is_background),
            Palette::Cyan => Cyan.to_escape_code(is_background),
            Palette::White => White.to_escape_code(is_background),
            Palette::LightBlack => LightBlack.to_escape_code(is_background),
            Palette::LightRed => LightRed.to_escape_code(is_background),
            Palette::LightGreen => LightGreen.to_escape_code(is_background),
            Palette::LightYellow => LightYellow.to_escape_code(is_background),
            Palette::LightBlue => LightBlue.to_escape_code(is_background),
            Palette::LightMagenta => LightMagenta.to_escape_code(is_background),
            Palette::LightCyan => LightCyan.to_escape_code(is_background),
            Palette::LightWhite => LightWhite.to_escape_code(is_background),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rgb() {
        assert_eq!(rgb(2, 3, 4).to_escape_code(true), "48;5;110m");
        assert_eq!(rgb(2, 1, 4).to_escape_code(false), "38;5;98m");
        assert_eq!(rgb(5, 1, 4).to_escape_code(false), "38;5;206m");
    }

    #[test]
    fn test_true_rgb() {
        assert_eq!(true_rgb(2, 3, 4).to_escape_code(true), "48;2;2;3;4m");
        assert_eq!(true_rgb(2, 1, 4).to_escape_code(false), "38;2;2;1;4m");
        assert_eq!(true_rgb(100, 1, 255).to_escape_code(false), "38;2;100;1;255m");
    }

    #[test]
    fn test_grayscale() {
        assert_eq!(grayscale(2).to_escape_code(true), "48;5;234m");
        assert_eq!(grayscale(5).to_escape_code(true), "48;5;237m");
    }

    #[test]
    fn test_normal() {
        assert_eq!(Black.to_escape_code(true), "48;5;0m");
        assert_eq!(Green.to_escape_code(true), "48;5;2m");
        assert_eq!(White.to_escape_code(true), "48;5;7m");
    }

    #[test]
    fn test_hi() {
        assert_eq!(LightRed.to_escape_code(true), "48;5;9m");
        assert_eq!(LightCyan.to_escape_code(true), "48;5;14m");
        assert_eq!(LightWhite.to_escape_code(true), "48;5;15m");
    }

    #[test]
    fn test_palette() {
        assert_eq!(Palette::Black.to_escape_code(false), Black.to_escape_code(false));
        assert_eq!(Palette::Red.to_escape_code(true), Red.to_escape_code(true));
        assert_eq!(Palette::LightBlue.to_escape_code(true), LightBlue.to_escape_code(true));
        assert_eq!(Palette::Rgb(2, 2, 2).to_escape_code(true), rgb(2, 2, 2).to_escape_code(true));
        assert_eq!(Palette::TrueRgb(5, 68, 255).to_escape_code(true), true_rgb(5, 68, 255).to_escape_code(true));
    }

    #[cfg(debug)]
    #[should_panic]
    #[test]
    fn test_bound_check_rgb() {
        rgb(3, 9, 1);
    }

    #[cfg(debug)]
    #[should_panic]
    #[test]
    fn test_bound_check_rgb_2() {
        rgb(3, 6, 1);
    }

    #[cfg(debug)]
    #[should_panic]
    #[test]
    fn test_bound_check_grayscale() {
        grayscale(25);
    }

    #[cfg(debug)]
    #[should_panic]
    #[test]
    fn test_palette_rgb_bound_check_1() {
        Palette::Rgb(3, 6, 1).to_escape_code(true);
    }

    #[cfg(debug)]
    #[should_panic]
    #[test]
    fn test_palette_rgb_bound_check_2() {
        Palette::Rgb(3, 9, 1).to_escape_code(true);
    }

    #[cfg(debug)]
    #[should_panic]
    #[test]
    fn test_palette_grayscale_bound_check_2() {
        Palette::Grayscale(25).to_escape_code(true);
    }
}
