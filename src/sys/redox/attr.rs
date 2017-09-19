use std::io;
use std::os::unix::io::RawFd;

use super::{cvt, syscall, Termios};

pub fn get_terminal_attr(fd: RawFd) -> io::Result<Termios> {
    let mut termios = Termios::default();

    let tfd = cvt(syscall::dup(fd, b"termios"))?;
    let res = cvt(syscall::read(tfd, &mut termios));
    let _ = syscall::close(tfd);

    if res? == termios.len() {
        Ok(termios)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Unable to get the terminal attributes."))
    }
}

pub fn set_terminal_attr(fd: RawFd, termios: &Termios) -> io::Result<()> {
    let tfd = cvt(syscall::dup(fd, b"termios"))?;
    let res = cvt(syscall::write(tfd, termios));
    let _ = syscall::close(tfd);

    if res? == termios.len() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Unable to set the terminal attributes."))
    }
}

pub fn raw_terminal_attr(ios: &mut Termios) {
    ios.make_raw()
}
