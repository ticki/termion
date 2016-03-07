use std::io::{Write, Result as IoResult};

/// Controlling terminals.
pub trait TermControl {
    /// Print the CSI (control sequence introducer) followed by a byte string.
    fn csi(&mut self, b: &[u8]) -> IoResult<usize>;
    /// Print OSC (operating system command) followed by a byte string.
    fn osc(&mut self, b: &[u8]) -> IoResult<usize>;
    /// Print OSC (device control string) followed by a byte string.
    fn dsc(&mut self, b: &[u8]) -> IoResult<usize>;
    /// Clear the terminal.
    fn clear(&mut self) -> IoResult<usize> {
        self.csi(b"2J")
    }
    /// Show the cursor.
    fn show(&mut self) -> IoResult<usize> {
        self.csi(b"?25h")
    }
    /// Hide the cursor.
    fn hide(&mut self) -> IoResult<usize> {
        self.csi(b"?25l")
    }
    /// Reset the style of the cursor.
    fn reset_style(&mut self) -> IoResult<usize> {
        self.csi(b"m")
    }
    /// Go to a given position.
    fn goto(&mut self, x: u16, y: u16) -> IoResult<usize> {
        self.csi(&[
             (x / 10000 % 10) as u8, (x / 1000 % 10) as u8, (x / 100 % 10) as u8, (x / 10 % 10) as u8, (x % 10) as u8,
             b';',
             (y / 10000 % 10) as u8, (y / 1000 % 10) as u8, (y / 100 % 10) as u8, (y / 10 % 10) as u8, (y % 10) as u8,
             b'H',
        ])
    }
}

impl<W: Write> TermControl for W {
    fn csi(&mut self, b: &[u8]) -> IoResult<usize> {
        self.write(b"\x1b[").and_then(|x| {
            self.write(b).map(|y| x + y)
        })
    }
    fn osc(&mut self, b: &[u8]) -> IoResult<usize> {
        self.write(b"\x1b]").and_then(|x| {
            self.write(b).map(|y| x + y)
        })
    }
    fn dsc(&mut self, b: &[u8]) -> IoResult<usize> {
        self.write(b"\x1bP").and_then(|x| {
            self.write(b).map(|y| x + y)
        })
    }
}
