use std::io::Write;
use std::ops::{Deref, DerefMut};

use TerminalError;

/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
#[cfg(target_os = "redox")]
pub struct RawTerminal<W> {
    output: W,
}

#[cfg(target_os = "redox")]
impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        use TermControl;
        self.csi(b"R");
    }
}

#[cfg(not(target_os = "redox"))]
use termios::Termios;
/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
#[cfg(not(target_os = "redox"))]
pub struct RawTerminal<W> {
    prev_ios: Termios,
    output: W,
}

#[cfg(not(target_os = "redox"))]
impl<W> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        use termios::set_terminal_attr;
        set_terminal_attr(&mut self.prev_ios as *mut _);
    }
}

impl<W> Deref for RawTerminal<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.output
    }
}
impl<W> DerefMut for RawTerminal<W> {
    fn deref_mut(&mut self) -> &mut W {
        &mut self.output
    }
}

/// Types which can be converted into "raw mode".
pub trait IntoRawMode: Sized {
    /// Switch to raw mode.
    ///
    /// Raw mode means that stdin won't be printed (it will instead have to be written manually by the
    /// program). Furthermore, the input isn't canonicalised or buffered (that is, you can read from
    /// stdin one byte of a time). The output is neither modified in any way.
    fn into_raw_mode(self) -> Result<RawTerminal<Self>, TerminalError>;
}

impl<W: Write> IntoRawMode for W {
    #[cfg(not(target_os = "redox"))]
    fn into_raw_mode(self) -> Result<RawTerminal<W>, TerminalError> {
        use termios::{cfmakeraw, get_terminal_attr, set_terminal_attr};

        let (mut ios, err) = get_terminal_attr();
        let prev_ios = ios.clone();
        if err != 0 {
            return Err(TerminalError::LoadAttrError);
        }

        unsafe {
            cfmakeraw(&mut ios);
        }

        if set_terminal_attr(&mut ios as *mut _) != 0 {
            Err(TerminalError::SetAttrError)
        } else {
            Ok(RawTerminal {
                prev_ios: prev_ios,
                output: self,
            })
        }
    }
    #[cfg(target_os = "redox")]
    fn into_raw_mode(self) -> Result<RawTerminal<W>, TerminalError> {
        use TermControl;

        if let Err(_) = self.csi("r") {
            Err(TerminalError::StdoutError)
        } else {
            Ok(RawTerminal {
                output: self,
            })
        }
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
