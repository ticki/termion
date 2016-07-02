/// A terminal color.
pub trait Color {
    /// Convert this to its ANSI value.
    fn to_ansi_val(self) -> u8;
}

macro_rules! derive_color {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        pub struct $name;

        impl Color for $name {
            #[inline]
            fn to_ansi_val(self) -> u8 {
                $value
            }
        }
    };
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
    fn to_ansi_val(self) -> u8 {
        self.0
    }
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
    /// Grayscale (max value: 24).
    Grayscale(u8),
}

impl Color for Palette {
    fn to_ansi_val(self) -> u8 {
        match self {
            Palette::Black => Black.to_ansi_val(),
            Palette::Red => Red.to_ansi_val(),
            Palette::Green => Green.to_ansi_val(),
            Palette::Yellow => Yellow.to_ansi_val(),
            Palette::Blue => Blue.to_ansi_val(),
            Palette::Magenta => Magenta.to_ansi_val(),
            Palette::Cyan => Cyan.to_ansi_val(),
            Palette::White => White.to_ansi_val(),
            Palette::LightBlack => LightBlack.to_ansi_val(),
            Palette::LightRed => LightRed.to_ansi_val(),
            Palette::LightGreen => LightGreen.to_ansi_val(),
            Palette::LightYellow => LightYellow.to_ansi_val(),
            Palette::LightBlue => LightBlue.to_ansi_val(),
            Palette::LightMagenta => LightMagenta.to_ansi_val(),
            Palette::LightCyan => LightCyan.to_ansi_val(),
            Palette::LightWhite => LightWhite.to_ansi_val(),
            Palette::Rgb(r, g, b) => rgb(r, g, b).to_ansi_val(),
            Palette::Grayscale(shade) => grayscale(shade).to_ansi_val(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rgb() {
        assert_eq!(rgb(2, 3, 4).to_ansi_val(), 110);
        assert_eq!(rgb(2, 1, 4).to_ansi_val(), 98);
        assert_eq!(rgb(5, 1, 4).to_ansi_val(), 206);
    }

    #[test]
    fn test_grayscale() {
        assert_eq!(grayscale(2).to_ansi_val(), 234);
        assert_eq!(grayscale(5).to_ansi_val(), 237);
    }

    #[test]
    fn test_normal() {
        assert_eq!(Black.to_ansi_val(), 0);
        assert_eq!(Green.to_ansi_val(), 2);
        assert_eq!(White.to_ansi_val(), 7);
    }

    #[test]
    fn test_hi() {
        assert_eq!(LightRed.to_ansi_val(), 9);
        assert_eq!(LightCyan.to_ansi_val(), 0xE);
        assert_eq!(LightWhite.to_ansi_val(), 0xF);
    }

    #[test]
    fn test_palette() {
        assert_eq!(Palette::Black.to_ansi_val(), Black.to_ansi_val());
        assert_eq!(Palette::Red.to_ansi_val(), Red.to_ansi_val());
        assert_eq!(Palette::LightBlue.to_ansi_val(), LightBlue.to_ansi_val());
        assert_eq!(Palette::Rgb(2, 2, 2).to_ansi_val(), rgb(2, 2, 2).to_ansi_val());
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
        Palette::Rgb(3, 6, 1).to_ansi_val();
    }

    #[cfg(debug)]
    #[should_panic]
    #[test]
    fn test_palette_rgb_bound_check_2() {
        Palette::Rgb(3, 9, 1).to_ansi_val();
    }

    #[cfg(debug)]
    #[should_panic]
    #[test]
    fn test_palette_grayscale_bound_check_2() {
        Palette::Grayscale(25).to_ansi_val();
    }
}
