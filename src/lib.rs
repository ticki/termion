#![feature(libc)]
extern crate libc;

use std::mem;
use self::libc::{c_int, c_uint, c_ushort, c_uchar, STDOUT_FILENO};
use self::libc::ioctl;

use std::io::{Write, Result as IoResult};

extern {
    static tiocgwinsz: c_int;

    fn tcgetattr(filedes: c_int, termptr: *mut Termios) -> c_int;
    fn tcsetattr(filedes: c_int, opt: c_int, termptr: *mut Termios) -> c_int;
    fn cfmakeraw(termptr: *mut Termios);
}

#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    _x: c_ushort,
    _y: c_ushort,
}

/// Get the size of the terminal. If the program isn't running in a terminal, it will return
/// `None`.
pub fn termsize() -> Option<(usize, usize)> {
    unsafe {
        let mut size: TermSize = mem::zeroed();

        if ioctl(STDOUT_FILENO, tiocgwinsz as u64, &mut size as *mut _) == 0 {
            Some((size.col as usize, size.row as usize))
        } else {
            None
        }
    }
}

#[derive(Clone)]
#[repr(C)]
struct Termios {
    c_iflag: c_uint,
    c_oflag: c_uint,
    c_cflag: c_uint,
    c_lflag: c_uint,
    c_line: c_uchar,
    c_cc: [c_uchar; 32],
    c_ispeed: c_uint,
    c_ospeed: c_uint,
}

fn get_terminal_attr() -> (Termios, c_int) {
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

fn make_raw(ios: &mut Termios) {
    unsafe {
        cfmakeraw(&mut *ios);
    }
}

fn set_terminal_attr(ios: *mut Termios) -> c_int {
    unsafe {
        tcsetattr(0, 0, ios)
    }
}

/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
pub struct TerminalRestorer {
    prev_ios: Termios
}

impl Drop for TerminalRestorer {
    fn drop(&mut self) {
        set_terminal_attr(&mut self.prev_ios as *mut _);
    }
}

/// Switch to raw mode.
///
/// Raw mode means that stdin won't be printed (it will instead have to be written manually by the
/// program). Furthermore, the input isn't canonicalised or buffered (that is, you can read from
/// stdin one byte of a time). The output is neither modified in any way.
///
/// Panics
/// ------
///
/// This may panic if the Termios settings can be set or loaded properly.
pub fn raw_mode() -> TerminalRestorer {
    let (mut ios, err) = get_terminal_attr();
    let prev_ios = ios.clone();
    if err != 0 {
        panic!("Failed to load termios settings properly.");
    }

    make_raw(&mut ios);

    if set_terminal_attr(&mut ios as *mut _) != 0 {
        panic!("Failed to init termios raw mode properly.");
    }

    TerminalRestorer {
        prev_ios: prev_ios,
    }
}

/// Controlling terminals.
pub trait TermControl {
    /// Print the CSI (control sequence introducer) followed by a byte string.
    fn csi(&mut self, b: &[u8]) -> IoResult<usize>;
    /// Clear the terminal.
    fn clear(&mut self) -> IoResult<usize> {
        self.csi(b"2J")
    }
    /// Show the cursor.
    fn show(&mut self) -> IoResult<usize> {
        self.csi(b"?25h")
    }
    /// Hide the cursor.
    fn hide(&mut self) -> IoResult<usize> {
        self.csi(b"?25l")
    }
    /// Reset the style of the cursor.
    fn reset_style(&mut self) -> IoResult<usize> {
        self.csi(b"0m")
    }
    /// Go to a given position.
    fn goto(&mut self, x: u16, y: u16) -> IoResult<usize> {
        self.csi(&[
             (x / 10000 % 10) as u8, (x / 1000 % 10) as u8, (x / 100 % 10) as u8, (x / 10 % 10) as u8, (x % 10) as u8,
             b';',
             (y / 10000 % 10) as u8, (y / 1000 % 10) as u8, (y / 100 % 10) as u8, (y / 10 % 10) as u8, (y % 10) as u8,
             b'H',
        ])
    }
}

impl<W: Write> TermControl for W {
    fn csi(&mut self, b: &[u8]) -> IoResult<usize> {
        self.write(b"\x1b[").and_then(|x| {
            self.write(b).map(|y| x + y)
        })
    }
}
