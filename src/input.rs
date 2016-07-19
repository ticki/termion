use std::io::{self, Read, Write};
use event::{parse_event, Event, Key};

use IntoRawMode;


/// An iterator over input keys.
pub struct Keys<I> {
    iter: Events<I>,
}

impl<I: Iterator<Item = Result<u8, io::Error>>> Iterator for Keys<I> {
    type Item = Result<Key, io::Error>;

    fn next(&mut self) -> Option<Result<Key, io::Error>> {
        loop {
            match self.iter.next() {
                Some(Ok(Event::Key(k))) => return Some(Ok(k)),
                Some(Ok(_)) => continue,
                e @ Some(Err(_)) => e,
                None => return None,
            };
        }
    }
}

/// An iterator over input events.
pub struct Events<I> {
    bytes: I,
}

impl<I: Iterator<Item = Result<u8, io::Error>>> Iterator for Events<I> {
    type Item = Result<Event, io::Error>;

    fn next(&mut self) -> Option<Result<Event, io::Error>> {
        let ref mut iter = self.bytes;
        match iter.next() {
            Some(item) => Some(parse_event(item, iter).or(Ok(Event::Unsupported))),
            None => None,
        }
    }
}

/// Extension to `Read` trait.
pub trait TermRead {
    /// An iterator over input events.
    fn events(self) -> Events<io::Bytes<Self>> where Self: Sized;

    /// An iterator over key inputs.
    fn keys(self) -> Keys<io::Bytes<Self>> where Self: Sized;

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
    fn events(self) -> Events<io::Bytes<R>> {
        Events {
            bytes: self.bytes(),
        }
    }
    fn keys(self) -> Keys<io::Bytes<R>> {
        Keys {
            iter: self.events(),
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

    #[test]
    fn test_function_keys() {
        let mut st = b"\x1BOP\x1BOQ\x1BOR\x1BOS".keys();
        for i in 1 .. 5 {
            assert_eq!(st.next().unwrap().unwrap(), Key::F(i));
        }

        let mut st = b"\x1B[11~\x1B[12~\x1B[13~\x1B[14~\x1B[15~\
        \x1B[17~\x1B[18~\x1B[19~\x1B[20~\x1B[21~\x1B[23~\x1B[24~".keys();
        for i in 1 .. 13 {
            assert_eq!(st.next().unwrap().unwrap(), Key::F(i));
        }
    }

    #[test]
    fn test_special_keys() {
        let mut st = b"\x1B[2~\x1B[H\x1B[7~\x1B[5~\x1B[3~\x1B[F\x1B[8~\x1B[6~".keys();
        assert_eq!(st.next().unwrap().unwrap(), Key::Insert);
        assert_eq!(st.next().unwrap().unwrap(), Key::Home);
        assert_eq!(st.next().unwrap().unwrap(), Key::Home);
        assert_eq!(st.next().unwrap().unwrap(), Key::PageUp);
        assert_eq!(st.next().unwrap().unwrap(), Key::Delete);
        assert_eq!(st.next().unwrap().unwrap(), Key::End);
        assert_eq!(st.next().unwrap().unwrap(), Key::End);
        assert_eq!(st.next().unwrap().unwrap(), Key::PageDown);
        assert!(st.next().is_none());
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
