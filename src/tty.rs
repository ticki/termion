use std::{fs, io};
use std::os::unix::io::AsRawFd;

/// Is this stream an TTY?
#[cfg(not(target_os = "redox"))]
pub fn is_tty<T: AsRawFd>(stream: &T) -> bool {
    use libc;

    unsafe { libc::isatty(stream.as_raw_fd()) == 1 }
}

/// This will panic.
#[cfg(target_os = "redox")]
pub fn is_tty<T: AsRawFd>(_stream: &T) -> bool {
    unimplemented!();
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
#[cfg(target_os = "redox")]
pub fn get_tty() -> io::Result<fs::File> {
    use std::env;
    let tty = try!(env::var("TTY").map_err(|x| io::Error::new(io::ErrorKind::NotFound, x)));
    fs::OpenOptions::new().read(true).write(true).open(tty)
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
#[cfg(not(target_os = "redox"))]
pub fn get_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(true).open("/dev/tty")
}
