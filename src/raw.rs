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
use cursor;

/// The timeout of an escape code control sequence, in milliseconds.
pub const CONTROL_SEQUENCE_TIMEOUT: u64 = 100;

/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
///
/// Restoring will entirely bring back the old TTY state.
#[cfg(target_os = "redox")]
pub struct RawTerminal<W: Write> {
    output: W,
}

#[cfg(target_os = "redox")]
impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        write!(self, csi!("?82l")).unwrap();
    }
}

#[cfg(not(target_os = "redox"))]
use termios::Termios;
/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
#[cfg(not(target_os = "redox"))]
pub struct RawTerminal<W: Write> {
    prev_ios: Termios,
    output: W,
}

#[cfg(not(target_os = "redox"))]
impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        // In the event that the cursor is hidden, this will show it before
        // exiting, preventing the parent process (most likely a shell) from
        // suppressing the cursor.
        write!(self, "{}", cursor::Show);

        use termios::set_terminal_attr;
        set_terminal_attr(&mut self.prev_ios as *mut _);
    }
}

impl<W: Write> ops::Deref for RawTerminal<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.output
    }
}

impl<W: Write> ops::DerefMut for RawTerminal<W> {
    fn deref_mut(&mut self) -> &mut W {
        &mut self.output
    }
}

impl<W: Write> Write for RawTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

/// Types which can be converted into "raw mode".
///
/// # Why is this type defined on writers and not readers?
///
/// TTYs has their state controlled by the writer, not the reader. You use the writer to clear the
/// screen, move the cursor and so on, so naturally you use the writer to change the mode as well.
pub trait IntoRawMode: Write + Sized {
    /// Switch to raw mode.
    ///
    /// Raw mode means that stdin won't be printed (it will instead have to be written manually by
    /// the program). Furthermore, the input isn't canonicalised or buffered (that is, you can
    /// read from stdin one byte of a time). The output is neither modified in any way.
    fn into_raw_mode(self) -> io::Result<RawTerminal<Self>>;
}

impl<W: Write> IntoRawMode for W {
    #[cfg(not(target_os = "redox"))]
    fn into_raw_mode(self) -> io::Result<RawTerminal<W>> {
        use termios::{cfmakeraw, get_terminal_attr, set_terminal_attr};

        let (mut ios, exit) = get_terminal_attr();
        let prev_ios = ios;
        if exit != 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Unable to get Termios attribute."));
        }

        unsafe {
            cfmakeraw(&mut ios);
        }

        if set_terminal_attr(&mut ios as *mut _) != 0 {
            Err(io::Error::new(io::ErrorKind::Other, "Unable to set Termios attribute."))
        } else {
            let res = RawTerminal {
                prev_ios: prev_ios,
                output: self,
            };
            Ok(res)
        }
    }

    #[cfg(target_os = "redox")]
    fn into_raw_mode(mut self) -> io::Result<RawTerminal<W>> {
        write!(self, csi!("?82h")).map(|_| RawTerminal { output: self })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{Write, stdout};

    #[test]
    fn test_into_raw_mode() {
        let mut out = stdout().into_raw_mode().unwrap();

        out.write_all(b"this is a test, muahhahahah").unwrap();
    }
}
