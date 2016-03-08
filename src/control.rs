use std::io::{Write, Result as IoResult};
use {Color, Style};

/// Controlling terminals.
pub trait TermControl {

    /// Print the CSI (control sequence introducer) followed by a byte string.
    fn csi(&mut self, b: &[u8]) -> IoResult<usize>;
    /// Print OSC (operating system command) followed by a byte string.
    fn osc(&mut self, b: &[u8]) -> IoResult<usize>;
    /// Print OSC (device control string) followed by a byte string.
    fn dsc(&mut self, b: &[u8]) -> IoResult<usize>;


    /// Clear the entire screen.
    fn clear(&mut self) -> IoResult<usize> {
        self.csi(b"2J")
    }
    /// Clear the current line.
    fn clear_line(&mut self) -> IoResult<usize> {
        self.csi(b"2K")
    }
    /// Show the cursor.
    fn show_cursor(&mut self) -> IoResult<usize> {
        self.csi(b"?25h")
    }
    /// Hide the cursor.
    fn hide_cursor(&mut self) -> IoResult<usize> {
        self.csi(b"?25l")
    }
    /// Reset the rendition mode.
    fn reset(&mut self) -> IoResult<usize> {
        self.csi(b"m")
    }
    /// Go to a given position.
    fn goto(&mut self, x: u16, y: u16) -> IoResult<usize> {
        self.csi(&[
            b'0' + (x / 10000) as u8,
            b'0' + (x / 1000) as u8 % 10,
            b'0' + (x / 100) as u8 % 10,
            b'0' + (x / 10) as u8 % 10,
            b'0' + x as u8 % 10,
            b';',
            b'0' + (y / 10000) as u8,
            b'0' + (y / 1000) as u8 % 10,
            b'0' + (y / 100) as u8 % 10,
            b'0' + (y / 10) as u8 % 10,
            b'0' + y as u8 % 10,
            b'H',
        ])
    }
    /// Set graphic rendition.
    fn rendition(&mut self, r: u8) -> IoResult<usize> {
        self.csi(&[
            b'0' + r / 100,
            b'0' + r / 10 % 10,
            b'0' + r % 10,
            b'm',
        ])
    }
    /// Set foreground color
    fn color(&mut self, color: Color) -> IoResult<usize> {
        let ansi = color.to_ansi_val();
        self.csi(&[
            b'3',
            b'8',
            b';',
            b'5',
            b';',
            b'0' + ansi / 100,
            b'0' + ansi / 10 % 10,
            b'0' + ansi % 10,
            b'm',
        ])
    }
    /// Set background color
    fn bg_color(&mut self, color: Color) -> IoResult<usize> {
        let ansi = color.to_ansi_val();
        self.csi(&[
            b'4',
            b'8',
            b';',
            b'5',
            b';',
            b'0' + ansi / 100,
            b'0' + ansi / 10 % 10,
            b'0' + ansi % 10,
            b'm',
        ])
    }
    /// Set rendition mode (SGR).
    fn style(&mut self, mode: Style) -> IoResult<usize> {
        self.rendition(mode as u8)
    }
}

impl<W: Write> TermControl for W {
    fn csi(&mut self, b: &[u8]) -> IoResult<usize> {
        self.write(b"\x1B[").and_then(|x| {
            self.write(b).map(|y| x + y)
        })
    }
    fn osc(&mut self, b: &[u8]) -> IoResult<usize> {
        self.write(b"\x1B]").and_then(|x| {
            self.write(b).map(|y| x + y)
        })
    }
    fn dsc(&mut self, b: &[u8]) -> IoResult<usize> {
        self.write(b"\x1BP").and_then(|x| {
            self.write(b).map(|y| x + y)
        })
    }
}
