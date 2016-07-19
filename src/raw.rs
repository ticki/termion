use std::io::{self, Write};
use std::ops::{Deref, DerefMut};

const ENTER_MOUSE_SEQUENCE: &'static[u8] = b"\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h";
const EXIT_MOUSE_SEQUENCE: &'static[u8] = b"\x1b[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l";

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
        try!(self.write(EXIT_MOUSE_SEQUENCE));
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
impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        use termios::set_terminal_attr;
        self.write(EXIT_MOUSE_SEQUENCE).unwrap();
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
    /// Raw mode means that stdin won't be printed (it will instead have to be written manually by the
    /// program). Furthermore, the input isn't canonicalised or buffered (that is, you can read from
    /// stdin one byte of a time). The output is neither modified in any way.
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
            let mut res = RawTerminal {
                prev_ios: prev_ios,
                output: self,
            };
            try!(res.write(ENTER_MOUSE_SEQUENCE));
            Ok(res)
        }
    }

    #[cfg(target_os = "redox")]
    fn into_raw_mode(mut self) -> io::Result<RawTerminal<W>> {
        use control::TermWrite;

        self.csi(b"r").map(|_| {
            let mut res = RawTerminal {
            output: self,
            };
            try!(res.write(ENTER_MOUSE_SEQUENCE));
            res
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

        out.write(b"this is a test, muahhahahah").unwrap();
    }
}
