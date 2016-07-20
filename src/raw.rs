use std::io::{self, Write};
use std::ops::{Deref, DerefMut};

/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
#[cfg(target_os = "redox")]
pub struct RawTerminal<W: Write> {
    output: W,
}

#[cfg(target_os = "redox")]
impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        use control::TermWrite;
        self.csi(b"R").unwrap();
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
impl<W> RawTerminal<W>
    where W: Write
{
    /// Enable mouse support.
    pub fn with_mouse(mut self) -> io::Result<MouseTerminal<W>> {
        try!(self.write(ENTER_MOUSE_SEQUENCE));
        Ok(MouseTerminal { term: self })
    }
}

#[cfg(not(target_os = "redox"))]
impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        use termios::set_terminal_attr;
        set_terminal_attr(&mut self.prev_ios as *mut _);
    }
}

impl<W: Write> Deref for RawTerminal<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.output
    }
}

impl<W: Write> DerefMut for RawTerminal<W> {
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
        let prev_ios = ios.clone();
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
        use control::TermWrite;

        self.csi(b"r").map(|_| {
            let mut res = RawTerminal { output: self };
            res
        })
    }
}

/// A sequence of escape codes to enable terminal mouse support.
const ENTER_MOUSE_SEQUENCE: &'static [u8] = b"\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h";

/// A sequence of escape codes to disable terminal mouse support.
const EXIT_MOUSE_SEQUENCE: &'static [u8] = b"\x1b[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l";

/// A `RawTerminal` with added mouse support.
///
/// To get such a terminal handle use `RawTerminal`'s
/// [`with_mouse()`](../termion/struct.RawTerminal.html#method.with_mouse) method.
#[cfg(not(target_os = "redox"))]
pub struct MouseTerminal<W: Write> {
    term: RawTerminal<W>,
}

#[cfg(not(target_os = "redox"))]
impl<W: Write> Drop for MouseTerminal<W> {
    fn drop(&mut self) {
        self.term.write(EXIT_MOUSE_SEQUENCE).unwrap();
    }
}

#[cfg(not(target_os = "redox"))]
impl<W: Write> Deref for MouseTerminal<W> {
    type Target = W;

    fn deref(&self) -> &W {
        self.term.deref()
    }
}

#[cfg(not(target_os = "redox"))]
impl<W: Write> DerefMut for MouseTerminal<W> {
    fn deref_mut(&mut self) -> &mut W {
        self.term.deref_mut()
    }
}

#[cfg(not(target_os = "redox"))]
impl<W: Write> Write for MouseTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.term.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.term.flush()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::{Write, stdout};

    #[test]
    fn test_into_raw_mode() {
        let mut out = stdout().into_raw_mode().unwrap();

        out.write(b"this is a test, muahhahahah").unwrap();
    }
}
