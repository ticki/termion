use libc::c_int;
use std::mem;

pub use libc::termios as Termios;

#[cfg(not(target_os = "macos"))]
pub const TIOCGWINSZ: usize = 0x00005413;

#[cfg(target_os = "macos")]
pub const TIOCGWINSZ: usize = 0x40087468;

extern {
    pub fn tcgetattr(fd: c_int, termptr: *mut Termios) -> c_int;
    pub fn tcsetattr(fd: c_int, opt: c_int, termptr: *mut Termios) -> c_int;
    pub fn cfmakeraw(termptr: *mut Termios);
}

pub fn get_terminal_attr() -> (Termios, c_int) {
    unsafe {
        let mut ios = mem::zeroed();
        let attr = tcgetattr(0, &mut ios);
        (ios, attr)
    }
}

pub fn set_terminal_attr(ios: *mut Termios) -> c_int {
    unsafe {
        tcsetattr(0, 0, ios)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_terminal_attr() {
        get_terminal_attr();
        get_terminal_attr();
        get_terminal_attr();
    }
    #[test]
    fn test_set_terminal_attr() {
        let mut ios = get_terminal_attr().0;
        set_terminal_attr(&mut ios as *mut _);
    }
}
