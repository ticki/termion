use std::io::{Read, Write};
use {IntoRawMode, TerminalError};

/// Extension to `Read` trait.
pub trait TermRead {
    /// Read a password.
    ///
    /// EOT and ETX will abort the prompt, returning `None`. Newline or carriage return will
    /// complete the password input.
    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> Result<Option<String>, TerminalError>;
}

impl<R: Read> TermRead for R {
    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> Result<Option<String>, TerminalError> {
        let _raw = try!(writer.into_raw_mode());
        let mut passbuf = Vec::with_capacity(30);

        for c in self.bytes() {
            match c {
                Err(_) => return Err(TerminalError::StdinError),
                Ok(0) | Ok(3) | Ok(4) => return Ok(None),
                Ok(b'\n') | Ok(b'\r') => break,
                Ok(c) => passbuf.push(c),
            }
        }

        let passwd = try!(String::from_utf8(passbuf).map_err(|_| TerminalError::UnicodeError));

        Ok(Some(passwd))
    }
}
