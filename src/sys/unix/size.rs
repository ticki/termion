use std::{io, mem};
use std::ffi::CString;

use super::cvt;
use super::libc::{c_ushort, ioctl, open, close, TIOCGWINSZ};

#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    _x: c_ushort,
    _y: c_ushort,
}
/// Get the size of the terminal.
pub fn terminal_size() -> io::Result<(u16, u16)> {
    let f = CString::new("/dev/tty").unwrap();
    unsafe {
        let mut size: TermSize = mem::zeroed();
        let fd = open(f.as_ptr(), 0);
        cvt(ioctl(fd, TIOCGWINSZ.into(), &mut size as *mut _))?;
        close(fd);
        Ok((size.col as u16, size.row as u16))
    }
}
