/// A SGR parameter (rendition mode).
pub enum Style {
    /// Reset SGR parameters.
    Reset = 0,
    /// Bold text.
    Bold = 1,
    /// Fainted text (not widely supported).
    Faint = 2,
    /// Italic text.
    Italic = 3,
    /// Underlined text.
    Underline = 4,
    /// Blinking text (not widely supported).
    Blink = 5,
    /// Inverted colors (negative mode).
    Invert = 7,
    /// Crossed out text (not widely supported).
    CrossedOut = 9,
    /// Framed text (not widely supported).
    Framed = 51,
}
