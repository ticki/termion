use std::{
    io, mem,
    os::fd::{AsRawFd, BorrowedFd},
};

use super::{cvt, Termios};

pub fn get_terminal_attr(fd: BorrowedFd) -> io::Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        cvt(libc::tcgetattr(fd.as_raw_fd(), &mut termios))?;
        Ok(termios)
    }
}

pub fn set_terminal_attr(fd: BorrowedFd, termios: &Termios) -> io::Result<()> {
    cvt(unsafe { libc::tcsetattr(fd.as_raw_fd(), libc::TCSANOW, termios) }).and(Ok(()))
}

pub fn raw_terminal_attr(termios: &mut Termios) {
    unsafe { libc::cfmakeraw(termios) }
}
