//! Managing raw mode.
//!
//! Raw mode is a particular state a TTY can have. It signifies that:
//!
//! 1. No line buffering (the input is given byte-by-byte).
//! 2. The input is not written out, instead it has to be done manually by the programmer.
//! 3. The output is not canonicalized (for example, `\n` means "go one line down", not "line
//!    break").
//!
//! It is essential to design terminal programs.
//!
//! # Example
//!
//! ```rust,no_run
//! use termion::raw::IntoRawMode;
//! use std::io::{Write, stdout};
//!
//! fn main() {
//!     let mut stdout = stdout().into_raw_mode().unwrap();
//!
//!     write!(stdout, "Hey there.").unwrap();
//! }
//! ```

use std::io::{self, Write};
use std::ops;
use std::os::unix::io::AsRawFd;

use sys::Termios;
use sys::attr::{get_terminal_attr, raw_terminal_attr, set_terminal_attr};

/// The timeout of an escape code control sequence, in milliseconds.
pub const CONTROL_SEQUENCE_TIMEOUT: u64 = 100;

/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
///
/// Restoring will entirely bring back the old TTY state.
pub struct RawTerminal<'a, W: IntoRawMode + 'a> {
    prev_ios: Termios,
    output: &'a mut W,
}

impl<'a, W: IntoRawMode> ops::Deref for RawTerminal<'a, W> {
    type Target = W;

    fn deref(&self) -> &W {
        self.output
    }
}

impl<'a, W: IntoRawMode> ops::DerefMut for RawTerminal<'a, W> {
    fn deref_mut(&mut self) -> &mut W {
        self.output
    }
}

impl<'a, W: IntoRawMode> Write for RawTerminal<'a, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl<'a, W: IntoRawMode> Drop for RawTerminal<'a, W> {
    fn drop(&mut self) {
        set_terminal_attr(self.output.as_raw_fd(), &self.prev_ios).unwrap();
    }
}

/// Types which can be converted into "raw mode".
///
/// # Why is this type defined on writers and not readers?
///
/// TTYs has their state controlled by the writer, not the reader. You use the writer to clear the
/// screen, move the cursor and so on, so naturally you use the writer to change the mode as well.
pub trait IntoRawMode: AsRawFd + Write + Sized {
    /// Switch to raw mode.
    ///
    /// Raw mode means that stdin won't be printed (it will instead have to be written manually by
    /// the program). Furthermore, the input isn't canonicalised or buffered (that is, you can
    /// read from stdin one byte of a time). The output is neither modified in any way.
    fn into_raw_mode<'a>(&'a mut self) -> io::Result<RawTerminal<Self>>;
}

impl<W: AsRawFd + Write> IntoRawMode for W {
    fn into_raw_mode<'a>(&'a mut self) -> io::Result<RawTerminal<W>> {
        let mut ios = get_terminal_attr(self.as_raw_fd())?;
        let prev_ios = ios;

        raw_terminal_attr(&mut ios);

        set_terminal_attr(self.as_raw_fd(), &ios)?;

        Ok(RawTerminal {
            prev_ios: prev_ios,
            output: self,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{Write, stdout};

    #[test]
    fn test_into_raw_mode() {
        let mut out = stdout().into_raw_mode().unwrap();

        out.write_all(b"this is a test, muahhahahah\r\n").unwrap();

        drop(out);
    }
}
