use std::io::{Error, ErrorKind};
use std::ascii::AsciiExt;
use std::str;

/// An event reported by the terminal.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    /// A key press.
    KeyEvent(Key),
    /// A mouse button press, release or wheel use at specific coordinates.
    MouseEvent(Mouse, u16, u16),
    /// An event that cannot currently be evaluated.
    Unsupported,
}

/// A mouse related event.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mouse {
    /// A mouse button was pressed.
    Press(MouseButton),
    /// A mouse button was released.
    Release,
}

/// A mouse button.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// Mouse wheel is going up.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelUp,
    /// Mouse wheel is going down.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelDown,
}

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
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,

    #[allow(missing_docs)]
    #[doc(hidden)]
    __IsNotComplete
}

pub fn parse_event<I>(item: Result<u8, Error>, iter: &mut I) -> Result<Event, Error>
where I: Iterator<Item = Result<u8, Error>>
{
    let error = Err(Error::new(ErrorKind::Other, "Could not parse an event"));
    match item {
        Ok(b'\x1B') => {
            Ok(match iter.next() {
                Some(Ok(b'O')) => {
                    match iter.next() {
                        Some(Ok(b'P')) => Event::KeyEvent(Key::F(1)),
                        Some(Ok(b'Q')) => Event::KeyEvent(Key::F(2)),
                        Some(Ok(b'R')) => Event::KeyEvent(Key::F(3)),
                        Some(Ok(b'S')) => Event::KeyEvent(Key::F(4)),
                        _ => return error,
                    }
                }
                Some(Ok(b'[')) => {
                    match iter.next() {
                        Some(Ok(b'D')) => Event::KeyEvent(Key::Left),
                        Some(Ok(b'C')) => Event::KeyEvent(Key::Right),
                        Some(Ok(b'A')) => Event::KeyEvent(Key::Up),
                        Some(Ok(b'B')) => Event::KeyEvent(Key::Down),
                        Some(Ok(b'H')) => Event::KeyEvent(Key::Home),
                        Some(Ok(b'F')) => Event::KeyEvent(Key::End),
                        Some(Ok(b'M')) => {
                            // X10 emulation mouse encoding: ESC [ CB Cx Cy (6 characters only)
                            let cb = iter.next().unwrap().unwrap() as i8 - 32;
                            // (1, 1) are the coords for upper left
                            let cx = (iter.next().unwrap().unwrap() as u8 - 1).saturating_sub(32);
                            let cy = (iter.next().unwrap().unwrap() as u8 - 1).saturating_sub(32);
                            Event::MouseEvent(match cb & 0b11 {
                                0 => {
                                    if cb & 64 != 0 {
                                        Mouse::Press(MouseButton::WheelUp)
                                    } else {
                                        Mouse::Press(MouseButton::Left)
                                    }
                                }
                                1 => {
                                    if cb & 64 != 0 {
                                        Mouse::Press(MouseButton::WheelDown)
                                    } else {
                                        Mouse::Press(MouseButton::Middle)
                                    }
                                }
                                2 => Mouse::Press(MouseButton::Right),
                                3 => Mouse::Release,
                                _ => return error,
                            },
                            cx as u16,
                            cy as u16)
                        }
                        Some(Ok(b'<')) => {
                            // xterm mouse encoding: ESC [ < Cb ; Cx ; Cy ; (M or m)
                            let mut buf = Vec::new();
                            let mut c = iter.next().unwrap().unwrap();
                                while match c {
                                    b'm' | b'M' => false,
                                    _ => true,
                                } {
                                    buf.push(c);
                                    c = iter.next().unwrap().unwrap();
                                }
                            let str_buf = String::from_utf8(buf).unwrap();
                            let ref mut nums = str_buf.split(';');

                            let cb = nums.next().unwrap().parse::<u16>().unwrap();
                            let cx = nums.next().unwrap().parse::<u16>().unwrap() - 1;
                            let cy = nums.next().unwrap().parse::<u16>().unwrap() - 1;

                            let button = match cb {
                                0 => MouseButton::Left,
                                1 => MouseButton::Middle,
                                2 => MouseButton::Right,
                                64 => MouseButton::WheelUp,
                                65 => MouseButton::WheelDown,
                                _ => return error,
                            };
                            Event::MouseEvent(match c {
                                b'M' => Mouse::Press(button),
                                b'm' => Mouse::Release,
                                _ => return error,

                            },
                            cx,
                            cy)
                        }
                        Some(Ok(c @ b'0'...b'9')) => {
                            // numbered escape code
                            let mut buf = Vec::new();
                            buf.push(c);
                            let mut c = iter.next().unwrap().unwrap();
                            while match c {
                                b'M' | b'~' => false,
                                _ => true,
                            } {
                                buf.push(c);
                                c = iter.next().unwrap().unwrap();
                            }

                            match c {
                                // rxvt mouse encoding: ESC [ Cb ; Cx ; Cy ; M
                                b'M' => {
                                    let str_buf = String::from_utf8(buf).unwrap();
                                    let ref mut nums = str_buf.split(';');

                                    let cb = nums.next().unwrap().parse::<u16>().unwrap();
                                    let cx = nums.next().unwrap().parse::<u16>().unwrap() - 1;
                                    let cy = nums.next().unwrap().parse::<u16>().unwrap() - 1;

                                    let event = match cb {
                                        32 => Mouse::Press(MouseButton::Left),
                                        33 => Mouse::Press(MouseButton::Middle),
                                        34 => Mouse::Press(MouseButton::Right),
                                        35 => Mouse::Release,
                                        96 => Mouse::Press(MouseButton::WheelUp),
                                        97 => Mouse::Press(MouseButton::WheelUp),
                                        _ => return error,
                                    };

                                    Event::MouseEvent(event, cx, cy)
                                },
                                // special key code
                                b'~' => {
                                    let num: u8 = String::from_utf8(buf).unwrap().parse().unwrap();
                                    match num {
                                        1 | 7 => Event::KeyEvent(Key::Home),
                                        2 => Event::KeyEvent(Key::Insert),
                                        3 => Event::KeyEvent(Key::Delete),
                                        4 | 8 => Event::KeyEvent(Key::End),
                                        5 => Event::KeyEvent(Key::PageUp),
                                        6 => Event::KeyEvent(Key::PageDown),
                                        v @ 11...15 => Event::KeyEvent(Key::F(v - 10)),
                                        v @ 17...21 => Event::KeyEvent(Key::F(v - 11)),
                                        v @ 23...24 => Event::KeyEvent(Key::F(v - 12)),
                                        _ => return error,
                                    }
                                }
                                _ => return error,
                            }
                        }
                        _ => return error,
                    }
                }
                Some(Ok(c)) => {
                    let ch = parse_utf8_char(c, iter);
                    Event::KeyEvent(Key::Alt(try!(ch)))
                }
                Some(Err(_)) | None => return error,
            })
        }
        Ok(b'\n') | Ok(b'\r') => Ok(Event::KeyEvent(Key::Char('\n'))),
        Ok(b'\t') => Ok(Event::KeyEvent(Key::Char('\t'))),
        Ok(b'\x7F') => Ok(Event::KeyEvent(Key::Backspace)),
        Ok(c @ b'\x01'...b'\x1A') => Ok(Event::KeyEvent(Key::Ctrl((c as u8 - 0x1 + b'a') as char))),
        Ok(c @ b'\x1C'...b'\x1F') => {
            Ok(Event::KeyEvent(Key::Ctrl((c as u8 - 0x1C + b'4') as char)))
        }
        Ok(b'\0') => Ok(Event::KeyEvent(Key::Null)),
        Ok(c) => {
            Ok({
                let ch = parse_utf8_char(c, iter);
                Event::KeyEvent(Key::Char(try!(ch)))
            })
        }
        Err(e) => Err(e),
    }
}

fn parse_utf8_char<I>(c: u8, iter: &mut I) -> Result<char, Error>
where I: Iterator<Item = Result<u8, Error>>
{
    let error = Err(Error::new(ErrorKind::Other, "Input character is not valid UTF-8"));
    if c.is_ascii() {
        Ok(c as char)
    } else {
        let ref mut bytes = Vec::new();
        bytes.push(c);

        loop {
            bytes.push(iter.next().unwrap().unwrap());
            match str::from_utf8(bytes) {
                Ok(st) => return Ok(st.chars().next().unwrap()),
                Err(_) => {},
            }
            if bytes.len() >= 4 { return error; }
        }
    }
}
