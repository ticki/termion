use libc::{c_int, c_uint, c_uchar};

#[cfg(not(target_os = "macos"))]
pub const TIOCGWINSZ: usize = 0x00005413;

#[cfg(target_os = "macos")]
pub const TIOCGWINSZ: usize = 0x40087468;

extern {
    pub fn tcgetattr(fd: c_int, termptr: *mut Termios) -> c_int;
    pub fn tcsetattr(fd: c_int, opt: c_int, termptr: *mut Termios) -> c_int;
    pub fn cfmakeraw(termptr: *mut Termios);
}

#[derive(Clone)]
#[repr(C)]
pub struct Termios {
    c_iflag: c_uint,
    c_oflag: c_uint,
    c_cflag: c_uint,
    c_lflag: c_uint,
    c_line: c_uchar,
    c_cc: [c_uchar; 32],
    c_ispeed: c_uint,
    c_ospeed: c_uint,
}

pub fn get_terminal_attr() -> (Termios, c_int) {
    unsafe {
        let mut ios = Termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0
        };

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
