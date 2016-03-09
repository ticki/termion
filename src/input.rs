use std::io::{Read, Write};
use {IntoRawMode, TerminalError};

#[cfg(feature = "nightly")]
use std::io::Chars;

/// A key.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Alt modified character.
    Alt(char),
    /// Normal character.
    Char(char),
    /// Invalid character code.
    Invalid,
    // TODO handle errors better?
    /// IO error.
    Error,
}

#[cfg(feature = "nightly")]
/// An iterator over input keys.
pub struct Keys<R> {
    chars: Chars<R>,
}

#[cfg(feature = "nightly")]
impl<R: Read> Iterator for Keys<R> {
    type Item = Key;

    fn next(&mut self) -> Option<Key> {
        match self.chars.next() {
            Some(Ok('\x1B')) => Some(match self.chars.next() {
                Some(Ok('[')) => match self.chars.next() {
                    Some(Ok('D')) => Key::Left,
                    Some(Ok('C')) => Key::Right,
                    Some(Ok('A')) => Key::Up,
                    Some(Ok('B')) => Key::Down,
                    _ => Key::Invalid,
                },
                Some(Ok(c)) => Key::Alt(c),
                Some(Err(_)) | None => Key::Invalid,
            }),
            Some(Ok('\x7F')) => Some(Key::Backspace),
            Some(Ok(c)) => Some(Key::Char(c)),
            None => None,
            Some(Err(_)) => Some(Key::Error),
        }
    }
}

/// Extension to `Read` trait.
pub trait TermRead {
    /// An iterator over key inputs.
    #[cfg(feature = "nightly")]
    fn keys(self) -> Keys<Self> where Self: Sized;

    /// Read a password.
    ///
    /// EOT and ETX will abort the prompt, returning `None`. Newline or carriage return will
    /// complete the password input.
    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> Result<Option<String>, TerminalError>;
}

impl<R: Read> TermRead for R {
    #[cfg(feature = "nightly")]
    fn keys(self) -> Keys<R> {
        Keys {
            chars: self.chars(),
        }
    }

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
