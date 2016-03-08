use std::io::{Read, Write};
use {IntoRawMode, TerminalError};

/// Extension to `Read` trait.
pub trait ReadExt {
    /// Read a password.
    ///
    /// EOT and ETX will abort the prompt, returning `None`. Newline or carriage return will
    /// complete the password input.
    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> Result<Option<String>, TerminalError>;
}

impl<R: Read> ReadExt for R {
    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> Result<Option<String>, TerminalError> {
        let _raw = try!(writer.into_raw_mode());
        let mut string = String::with_capacity(30);

        for c in self.bytes() {
            match c {
                Err(_) => return Err(TerminalError::StdinError),
                Ok(0) | Ok(3) | Ok(4) => return Ok(None),
                Ok(b'\n') | Ok(b'\r') => return Ok(Some(string)),
                Ok(c) => string.push(c as char),
            }
        }

        Ok(Some(string))
    }
}
