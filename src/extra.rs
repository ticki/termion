use std::io::Read;
use raw_mode;

/// Extension to `Read` trait.
pub trait ReadExt {
    /// Read a password.
    fn read_passwd(&mut self) -> Option<String>;
}

impl<R: Read> ReadExt for R {
    fn read_passwd(&mut self) -> Option<String> {
        let _raw = raw_mode();
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
