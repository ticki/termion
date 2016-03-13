use std::io::{Error, ErrorKind, Result as IoResult};

#[cfg(not(target_os = "redox"))]
use libc::c_ushort;

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
pub fn terminal_size() -> IoResult<(usize, usize)> {
    use libc::ioctl;
    use libc::STDOUT_FILENO;

    use std::mem;
    unsafe {
        let mut size: TermSize = mem::zeroed();

        if ioctl(STDOUT_FILENO, tiocgwinsz(), &mut size as *mut _) == 0 {
            Ok((size.col as usize, size.row as usize))
        } else {
            Err(Error::new(ErrorKind::Other, "Unable to get the terminal size."))
        }
    }
}

/// Get the size of the terminal.
#[cfg(target_os = "redox")]
pub fn terminal_size() -> IoResult<(usize, usize), TerminalError> {
    fn get_int(s: &'static str) -> IoResult<usize> {
        use std::env::{VarError, var};

        var(s).map_err(|e| match e {
            VarError::NotPresent => Error::new(ErrorKind::NotFound, e),
            VarError::NotUnicode(u) => Error::new(ErrorKind::InvalidData, u),
        }).and_then(|x| {
            x.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))
        })
    }

    Ok((try!(get_int("COLUMNS")), try!(get_int("LINES"))))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size() {
        assert!(terminal_size().is_ok());
    }
}
