use std::io;

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
#[cfg(not(target_os = "redox"))]
#[cfg(target_pointer_width = "64")]
fn tiocgwinsz() -> u64 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as u64
}
#[cfg(not(target_os = "redox"))]
#[cfg(target_pointer_width = "32")]
fn tiocgwinsz() -> u32 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as u32
}


/// Get the size of the terminal.
#[cfg(not(target_os = "redox"))]
pub fn terminal_size() -> io::Result<(usize, usize)> {
    use libc::ioctl;
    use libc::STDOUT_FILENO;

    use std::mem;
    unsafe {
        let mut size: TermSize = mem::zeroed();

        if ioctl(STDOUT_FILENO, tiocgwinsz(), &mut size as *mut _) == 0 {
            Ok((size.col as usize, size.row as usize))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Unable to get the terminal size."))
        }
    }
}

/// Get the size of the terminal.
#[cfg(target_os = "redox")]
pub fn terminal_size() -> io::Result<(usize, usize)> {
    use std::env;

    let width = env::var("COLUMNS").unwrap_or(String::new()).parse::<usize>().unwrap_or(0);
    let height = env::var("LINES").unwrap_or(String::new()).parse::<usize>().unwrap_or(0);

    Ok((width, height))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size() {
        assert!(terminal_size().is_ok());
    }
}
