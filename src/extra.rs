use std::io::{Read, Write};
use IntoRawMode;

/// Extension to `Read` trait.
pub trait ReadExt {
    /// Read a password.
    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> Option<String>;
}

impl<R: Read> ReadExt for R {
    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> Option<String> {
        let _raw = if let Ok(x) = writer.into_raw_mode() {
            x
        } else {
            return None;
        };
        let mut string = String::with_capacity(30);

        for c in self.chars() {
            match if let Ok(c) = c {
                c
            } else {
                return None;
            } {
                '\x00' | '\x03' | '\x04' => return None,
                '\r' => return Some(string),
                b => string.push(b),
            }
        }

        Some(string)
    }
}
