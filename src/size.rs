use libc::ioctl;
use libc::{c_ushort, STDOUT_FILENO};

use std::mem;

use termios::{TermiosError, tiocgwinsz};

#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    _x: c_ushort,
    _y: c_ushort,
}

/// Get the size of the terminal. If the program isn't running in a terminal, it will return
/// `None`.
pub fn termsize() -> Result<(usize, usize), TermiosError> {
    unsafe {
        let mut size: TermSize = mem::zeroed();

        if ioctl(STDOUT_FILENO, tiocgwinsz as u64, &mut size as *mut _) == 0 {
            Ok((size.col as usize, size.row as usize))
        } else {
            Err(TermiosError::TermSizeError)
        }
    }
}
