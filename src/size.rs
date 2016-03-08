#[cfg(not(target_os = "redox"))]
use libc::c_ushort;

use TerminalError;

#[cfg(not(target_os = "redox"))]
#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    _x: c_ushort,
    _y: c_ushort,
}

/// Get the size of the terminal.
#[cfg(not(target_os = "redox"))]
pub fn terminal_size() -> Result<(usize, usize), TerminalError> {
    use libc::ioctl;
    use libc::STDOUT_FILENO;
    use termios::TIOCGWINSZ;

    use std::mem;
    unsafe {
        let mut size: TermSize = mem::zeroed();

        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size as *mut _) == 0 {
            Ok((size.col as usize, size.row as usize))
        } else {
            Err(TerminalError::TermSizeError)
        }
    }
}

/// Get the size of the terminal.
#[cfg(target_os = "redox")]
pub fn terminal_size() -> Result<(usize, usize), TerminalError> {
    use std::env::var;

    let w = var("COLUMNS").map_err(|_| TerminalError::TermSizeError).and_then(|x| {
        x.parse().map_err(|_| TerminalError::ParseError)
    });
    let h = var("LINES").map_err(|_| TerminalError::TermSizeError).and_then(|x| {
        x.parse().map_err(|_| TerminalError::ParseError)
    });

    Ok((try!(w), try!(h)))
}
