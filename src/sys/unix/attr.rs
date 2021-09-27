use std::{io, mem};

use super::{cvt, Termios};

pub fn get_terminal_attr() -> io::Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        cvt(libc::tcgetattr(1, &mut termios))?;
        Ok(termios)
    }
}

pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
    cvt(unsafe { libc::tcsetattr(1, 0, termios) }).and(Ok(()))
}

pub fn raw_terminal_attr(termios: &mut Termios) {
    unsafe { libc::cfmakeraw(termios) }
}
