//! Input.

use std::io::{self, Read, Write};

use raw::IntoRawMode;

/// A key.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Invalid character code.
    Invalid,
    /// Null byte.
    Null,

    #[allow(missing_docs)]
    #[doc(hidden)]
    __IsNotComplete
}

/// An iterator over input keys.
#[cfg(feature = "nightly")]
pub struct Keys<I> {
    chars: I,
}

#[cfg(feature = "nightly")]
impl<I: Iterator<Item = Result<char, io::CharsError>>> Iterator for Keys<I> {
    type Item = Result<Key, io::CharsError>;

    fn next(&mut self) -> Option<Result<Key, io::CharsError>> {
        Some(match self.chars.next() {
            Some(Ok('\x1B')) => Ok(match self.chars.next() {
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
            Some(Ok('\n')) | Some(Ok('\r')) => Ok(Key::Char('\n')),
            Some(Ok('\t')) => Ok(Key::Char('\t')),
            Some(Ok('\x7F')) => Ok(Key::Backspace),
            Some(Ok(c @ '\x01' ... '\x1A')) => Ok(Key::Ctrl((c as u8 - 0x1  + b'a') as char)),
            Some(Ok(c @ '\x1C' ... '\x1F')) => Ok(Key::Ctrl((c as u8 - 0x1C + b'4') as char)),
            Some(Ok('\0')) => Ok(Key::Null),
            Some(Ok(c)) => Ok(Key::Char(c)),
            Some(Err(e)) => Err(e),
            None => return None,
        })
    }
}

/// Extension to `Read` trait.
pub trait TermRead {
    /// An iterator over key inputs.
    #[cfg(feature = "nightly")]
    fn keys(self) -> Keys<io::Chars<Self>> where Self: Sized;

    /// Read a line.
    ///
    /// EOT and ETX will abort the prompt, returning `None`. Newline or carriage return will
    /// complete the input.
    fn read_line(&mut self) -> io::Result<Option<String>>;

    /// Read a password.
    ///
    /// EOT and ETX will abort the prompt, returning `None`. Newline or carriage return will
    /// complete the input.
    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> io::Result<Option<String>> {
        let _raw = try!(writer.into_raw_mode());
        self.read_line()
    }
}


impl<R: Read> TermRead for R {
    #[cfg(feature = "nightly")]
    fn keys(self) -> Keys<io::Chars<R>> {
        Keys {
            chars: self.chars(),
        }
    }

    fn read_line(&mut self) -> io::Result<Option<String>> {
        let mut buf = Vec::with_capacity(30);

        for c in self.bytes() {
            match c {
                Err(e) => return Err(e),
                Ok(0) | Ok(3) | Ok(4) => return Ok(None),
                Ok(0x7f) => { buf.pop(); },
                Ok(b'\n') | Ok(b'\r') => break,
                Ok(c) => buf.push(c),
            }
        }

        let string = try!(String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));
        Ok(Some(string))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;

    #[cfg(feature = "nightly")]
    #[test]
    fn test_keys() {
        let mut i = b"\x1Bayo\x7F\x1B[D".keys();

        assert_eq!(i.next().unwrap().unwrap(), Key::Alt('a'));
        assert_eq!(i.next().unwrap().unwrap(), Key::Char('y'));
        assert_eq!(i.next().unwrap().unwrap(), Key::Char('o'));
        assert_eq!(i.next().unwrap().unwrap(), Key::Backspace);
        assert_eq!(i.next().unwrap().unwrap(), Key::Left);
        assert!(i.next().is_none());
    }

    fn line_match(a: &str, b: Option<&str>) {
        let mut sink = io::sink();

        let line = a.as_bytes().read_line().unwrap();
        let pass = a.as_bytes().read_passwd(&mut sink).unwrap();

        // godammit rustc

        assert_eq!(line, pass);

        if let Some(l) = line {
            assert_eq!(Some(l.as_str()), b);
        } else {
            assert!(b.is_none());
        }
    }

    #[test]
    fn test_read() {
        let test1 = "this is the first test";
        let test2 = "this is the second test";

        line_match(test1, Some(test1));
        line_match(test2, Some(test2));
    }

    #[test]
    fn test_backspace() {
        line_match("this is the\x7f first\x7f\x7f test", Some("this is th fir test"));
        line_match("this is the seco\x7fnd test\x7f", Some("this is the secnd tes"));
    }

    #[test]
    fn test_end() {
        line_match("abc\nhttps://www.youtube.com/watch?v=dQw4w9WgXcQ", Some("abc"));
        line_match("hello\rhttps://www.youtube.com/watch?v=yPYZpwSpKmA", Some("hello"));
    }

    #[test]
    fn test_abort() {
        line_match("abc\x03https://www.youtube.com/watch?v=dQw4w9WgXcQ", None);
        line_match("hello\x04https://www.youtube.com/watch?v=yPYZpwSpKmA", None);
    }
}
