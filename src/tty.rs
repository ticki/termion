use std::os::unix::io::AsRawFd;

/// Is this stream an TTY?
#[cfg(not(target_os = "redox"))]
pub fn is_tty<T: AsRawFd>(stream: T) -> bool {
    use libc;

    unsafe { libc::isatty(stream.as_raw_fd()) == 1}
}

/// This will panic.
#[cfg(target_os = "redox")]
pub fn is_tty<T: AsRawFd>(_stream: T) -> bool {
    unimplemented!();
}
