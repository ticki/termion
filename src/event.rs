//! Mouse and key events.

use std::io::{Error, ErrorKind};
use std::str;

/// An event reported by the terminal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    /// A key press.
    Key(Key),
    /// A mouse button press, release or wheel use at specific coordinates.
    Mouse(MouseEvent),
    /// An event that cannot currently be evaluated.
    Unsupported(Vec<u8>),
}

/// A mouse related event.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseEvent {
    /// A mouse button was pressed.
    ///
    /// The coordinates are one-based.
    Press(MouseButton, u16, u16),
    /// A mouse button was released.
    ///
    /// The coordinates are one-based.
    Release(u16, u16),
    /// A mouse button is held over the given coordinates.
    ///
    /// The coordinates are one-based.
    Hold(u16, u16),
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
    /// Esc key.
    Esc,

    #[doc(hidden)]
    __IsNotComplete,
}

/// Parse an Event from `item` and possibly subsequent bytes through `iter`.
///
/// Note that this will /not/ parse `\x1B` as `Key::Esc`, since we can't ensure that a
/// control sequence may follow, and if checked we'd potentially lose a byte of input.
pub fn parse_event<I>(item: u8, iter: &mut I) -> Result<Event, Error>
where
    I: Iterator<Item = Result<u8, Error>>,
{
    match item {
        b'\x1B' => {
            // This is an escape character, leading a control sequence.
            match iter.next() {
                Some(Ok(b'O')) => {
                    match iter.next() {
                        // F1-F4
                        Some(Ok(val @ b'P'...b'S')) => Ok(Event::Key(Key::F(1 + val - b'P'))),
                        _ => Err(Error::new(ErrorKind::Other, "Could not parse an event")),
                    }
                }
                // This is a CSI sequence.
                Some(Ok(b'[')) => parse_csi(iter),
                Some(Ok(c)) => Ok(Event::Key(Key::Alt(parse_utf8_char(c, iter)?))),
                Some(Err(_)) | None => {
                    Err(Error::new(ErrorKind::Other, "Could not parse an event"))
                }
            }
        }
        b'\n' | b'\r' => Ok(Event::Key(Key::Char('\n'))),
        b'\t' => Ok(Event::Key(Key::Char('\t'))),
        b'\x7F' => Ok(Event::Key(Key::Backspace)),
        c @ b'\x01'...b'\x1A' => Ok(Event::Key(Key::Ctrl((c as u8 - 0x1 + b'a') as char))),
        c @ b'\x1C'...b'\x1F' => Ok(Event::Key(Key::Ctrl((c as u8 - 0x1C + b'4') as char))),
        b'\0' => Ok(Event::Key(Key::Null)),
        c => Ok(Event::Key(Key::Char(parse_utf8_char(c, iter)?))),
    }
}

/// Parses a CSI sequence, just after reading ^[
///
/// Returns None if an unrecognized sequence is found.
fn parse_csi<I>(iter: &mut I) -> Result<Event, Error>
where
    I: Iterator<Item = Result<u8, Error>>,
{
    Ok(match iter.next() {
        Some(Ok(b'[')) => match iter.next() {
            Some(Ok(val @ b'A'...b'E')) => Event::Key(Key::F(1 + val - b'A')),
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid CSI sequence")),
        },
        Some(Ok(b'D')) => Event::Key(Key::Left),
        Some(Ok(b'C')) => Event::Key(Key::Right),
        Some(Ok(b'A')) => Event::Key(Key::Up),
        Some(Ok(b'B')) => Event::Key(Key::Down),
        Some(Ok(b'H')) => Event::Key(Key::Home),
        Some(Ok(b'F')) => Event::Key(Key::End),
        // X10 emulation mouse encoding: ESC [ CB Cx Cy (6 characters only).
        Some(Ok(b'M')) => {
            let mut next = || {
                iter.next()
                    .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid CSI Seqeunce"))?
            };

            let cb = next()? as i8 - 32;
            // (1, 1) are the coords for upper left.
            let cx = next()?.saturating_sub(32) as u16;
            let cy = next()?.saturating_sub(32) as u16;
            Event::Mouse(match cb & 0b11 {
                0 => {
                    if cb & 0x40 != 0 {
                        MouseEvent::Press(MouseButton::WheelUp, cx, cy)
                    } else {
                        MouseEvent::Press(MouseButton::Left, cx, cy)
                    }
                }
                1 => {
                    if cb & 0x40 != 0 {
                        MouseEvent::Press(MouseButton::WheelDown, cx, cy)
                    } else {
                        MouseEvent::Press(MouseButton::Middle, cx, cy)
                    }
                }
                2 => MouseEvent::Press(MouseButton::Right, cx, cy),
                3 => MouseEvent::Release(cx, cy),
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid CSI sequence")),
            })
        }
        // xterm mouse encoding:
        // ESC [ < Cb ; Cx ; Cy (;) (M or m)
        Some(Ok(b'<')) => {
            // Parse coordinate bytes, each coordinate is a u16
            let mut next = |break_early| -> Result<(u16, u8), Error> {
                let mut len = 0;
                let mut end = b'\0';
                let mut bytes = [b'\0'; 5];
                for i in 0..5 {
                    match iter.next().ok_or_else(|| {
                        Error::new(ErrorKind::InvalidData, "Incomplete CSI sequence")
                    })?? {
                        b';' => {
                            len = i;
                            if break_early {
                                break;
                            }
                        }
                        c @ b'm' | c @ b'M' if !break_early => {
                            if len == 0 {
                                len = i;
                            }
                            end = c;
                            break;
                        }
                        c => {
                            bytes[i] = c;
                        }
                    }
                }
                Ok((
                    str::from_utf8(&bytes[..len])
                        .map_err(|_| {
                            Error::new(ErrorKind::InvalidData, "CSI sequence is not valid UTF-8")
                        })?
                        .parse()
                        .map_err(|_| {
                            Error::new(ErrorKind::InvalidData, "CSI sequence contains invalid u16")
                        })?,
                    end,
                ))
            };

            let (cb, cx, (cy, end)) = (next(true)?.0, next(true)?.0, next(false)?);
            let event = match cb {
                0...2 | 64...65 => {
                    let button = match cb {
                        0 => MouseButton::Left,
                        1 => MouseButton::Middle,
                        2 => MouseButton::Right,
                        64 => MouseButton::WheelUp,
                        65 => MouseButton::WheelDown,
                        _ => unreachable!(),
                    };
                    match end {
                        b'M' => MouseEvent::Press(button, cx, cy),
                        b'm' => MouseEvent::Release(cx, cy),
                        _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid CSI sequence")),
                    }
                }
                32 => MouseEvent::Hold(cx, cy),
                3 => MouseEvent::Release(cx, cy),
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid CSI sequence")),
            };

            Event::Mouse(event)
        }
        // Numbered escape code.
        Some(Ok(c @ b'0'...b'9')) => {
            let mut count = 0;
            let mut bytes = [b'\0'; 20];
            bytes[0] = c;

            for i in 1..20 {
                match iter.next().ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "Incomplete CSI sequence")
                })?? {
                    c @ b'M' | c @ b'~' => {
                        bytes[i] = c;
                        count = i;
                        break;
                    }
                    c @ _ => {
                        bytes[i] = c;
                    }
                }
            }

            match bytes[count] {
                // rxvt mouse encoding:
                // ESC [ Cb ; Cx ; Cy ; M
                b'M' => {
                    let mut next = |offset| -> Result<(u16, _), Error> {
                        let mut len = 0;
                        for i in 1..5 {
                            match bytes[offset + i] {
                                b';' | b'M' | b'\0' => {
                                    len = i;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        Ok((
                            str::from_utf8(&bytes[offset..(offset + len)])
                                .map_err(|_| {
                                    Error::new(
                                        ErrorKind::InvalidData,
                                        "CSI sequence is not valid UTF-8",
                                    )
                                })?
                                .parse()
                                .map_err(|e| {
                                    Error::new(
                                        ErrorKind::InvalidData,
                                        "CSI sequence contains invalid u16",
                                    )
                                })?,
                            offset + len + 1,
                        ))
                    };

                    let (cb, off) = next(0)?;
                    let (cx, off) = next(off)?;
                    let (cy, _) = next(off)?;

                    let event = match cb {
                        32 => MouseEvent::Press(MouseButton::Left, cx, cy),
                        33 => MouseEvent::Press(MouseButton::Middle, cx, cy),
                        34 => MouseEvent::Press(MouseButton::Right, cx, cy),
                        35 => MouseEvent::Release(cx, cy),
                        64 => MouseEvent::Hold(cx, cy),
                        96 | 97 => MouseEvent::Press(MouseButton::WheelUp, cx, cy),
                        _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid CSI sequence")),
                    };

                    Event::Mouse(event)
                }
                // Special key code.
                // This CSI sequence can be a list of semicolon-separated numbers.
                b'~' => {
                    let mut next = |offset| -> Result<(u8, _), Error> {
                        let mut len = 0;
                        for i in 1..3 {
                            match bytes[offset + i] {
                                b';' | b'~' | b'\0' => {
                                    len = i;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        Ok((
                            str::from_utf8(&bytes[offset..offset + len])
                                .map_err(|_| {
                                    Error::new(
                                        ErrorKind::InvalidData,
                                        "CSI sequence is not valid UTF-8",
                                    )
                                })?
                                .parse()
                                .map_err(|_| {
                                    Error::new(
                                        ErrorKind::InvalidData,
                                        "CSI sequence contains invalid u16",
                                    )
                                })?,
                            offset + len,
                        ))
                    };

                    let (num, off) = next(0)?;

                    // TODO: handle multiple values for key modifiers (ex: values
                    // [3, 2] means Shift+Delete)
                    if next(off).is_ok() {
                        return Err(Error::new(ErrorKind::Other, "CSI sequences with a special key code andmultiple key modifiers not yet supported"));
                    }

                    match num {
                        1 | 7 => Event::Key(Key::Home),
                        2 => Event::Key(Key::Insert),
                        3 => Event::Key(Key::Delete),
                        4 | 8 => Event::Key(Key::End),
                        5 => Event::Key(Key::PageUp),
                        6 => Event::Key(Key::PageDown),
                        v @ 11...15 => Event::Key(Key::F(v - 10)),
                        v @ 17...21 => Event::Key(Key::F(v - 11)),
                        v @ 23...24 => Event::Key(Key::F(v - 12)),
                        _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid CSI sequence")),
                    }
                }
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid CSI sequence")),
            }
        }
        _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid CSI sequence")),
    })
}

/// Parse `c` as either a single byte ASCII char or a variable size UTF-8 char.
fn parse_utf8_char<I>(c: u8, iter: &mut I) -> Result<char, Error>
where
    I: Iterator<Item = Result<u8, Error>>,
{
    if c.is_ascii() {
        Ok(c as char)
    } else {
        let mut bytes = [c, b'\0', b'\0', b'\0'];
        for i in 1..4 {
            match iter.next() {
                Some(Ok(next)) => {
                    bytes[i] = next;
                    if let Ok(st) = str::from_utf8(&bytes[..i + 1]) {
                        return Ok(st.chars().next().unwrap());
                    }
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Input character is not valid UTF-8",
                    ))
                }
            }
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            "Input character is not valid UTF-8",
        ))
    }
}

#[cfg(test)]
#[test]
fn test_parse_utf8() {
    let st = "abcéŷ¤£€ù%323";
    let ref mut bytes = st.bytes().map(|x| Ok(x));
    let chars = st.chars();
    for c in chars {
        let b = bytes.next().unwrap().unwrap();
        assert!(c == parse_utf8_char(b, bytes).unwrap());
    }
}
