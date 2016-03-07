use libc::{c_int, c_uint, c_uchar};

use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};

extern {
    pub static tiocgwinsz: c_int;

    pub fn tcgetattr(filedes: c_int, termptr: *mut Termios) -> c_int;
    pub fn tcsetattr(filedes: c_int, opt: c_int, termptr: *mut Termios) -> c_int;
    pub fn cfmakeraw(termptr: *mut Termios);
}

#[derive(Clone)]
#[repr(C)]
pub struct Termios {
    c_iflag: c_uint,
    c_oflag: c_uint,
    c_cflag: c_uint,
    c_lflag: c_uint,
    c_line: c_uchar,
    c_cc: [c_uchar; 32],
    c_ispeed: c_uint,
    c_ospeed: c_uint,
}

pub fn get_terminal_attr() -> (Termios, c_int) {
    unsafe {
        let mut ios = Termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0
        };

        let attr = tcgetattr(0, &mut ios);
        (ios, attr)
    }
}

pub fn set_terminal_attr(ios: *mut Termios) -> c_int {
    unsafe {
        tcsetattr(0, 0, ios)
    }
}

/// Termios error.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TermiosError {
    /// Failed to load attributes.
    LoadAttrError,
    /// Failed to set attributes.
    SetAttrError,
    /// Failed to get terminal size.
    TermSizeError,
}

impl TermiosError {
    fn msg(self) -> &'static str {
        match self {
            TermiosError::LoadAttrError => "Failed to load Termios attributes.",
            TermiosError::SetAttrError => "Failed to set Termios attribute.",
            TermiosError::TermSizeError => "Failed to get terminal size.",
        }
    }
}

impl Error for TermiosError {
    fn description(&self) -> &str {
        self.msg()
    }
}

impl Display for TermiosError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.write_str(self.msg())
    }
}
