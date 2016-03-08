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

// Since attributes on non-item statements is not stable yet, we use a function.
#[cfg(target_pointer_width = "64")]
fn tiocgwinsz() -> u64 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as u64
}
#[cfg(target_pointer_width = "32")]
fn tiocgwinsz() -> u32 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as u32
}


/// Get the size of the terminal.
#[cfg(not(target_os = "redox"))]
pub fn terminal_size() -> Result<(usize, usize), TerminalError> {
    use libc::ioctl;
    use libc::STDOUT_FILENO;

    use std::mem;
    unsafe {
        let mut size: TermSize = mem::zeroed();

        if ioctl(STDOUT_FILENO, tiocgwinsz(), &mut size as *mut _) == 0 {
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
