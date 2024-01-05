use std::convert::TryInto;
use std::io;
use std::os::fd::{AsRawFd, BorrowedFd};

use super::{cvt, Termios};

pub fn get_terminal_attr(fd: BorrowedFd) -> io::Result<Termios> {
    let mut termios = Termios::default();

    let fd: usize = fd
        .as_raw_fd()
        .try_into()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let fd = cvt(libredox::call::dup(fd, b"termios"))?;
    let res = cvt(libredox::call::read(fd, &mut termios));
    let _ = libredox::call::close(fd);

    if res? == termios.len() {
        Ok(termios)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to get the terminal attributes.",
        ))
    }
}

pub fn set_terminal_attr(fd: BorrowedFd, termios: &Termios) -> io::Result<()> {
    let fd: usize = fd
        .as_raw_fd()
        .try_into()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let fd = cvt(libredox::call::dup(fd, b"termios"))?;
    let res = cvt(libredox::call::write(fd, termios));
    let _ = libredox::call::close(fd);

    if res? == termios.len() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to set the terminal attributes.",
        ))
    }
}

pub fn raw_terminal_attr(ios: &mut Termios) {
    ios.make_raw()
}
