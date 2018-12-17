use std::{fs, io::{self, Read}};
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle};

/// Is this stream a TTY?
pub fn is_tty<T: AsRawHandle>(stream: &T) -> bool {
    // @MAYBE Jezza - 17 Dec. 2018: Is this the correct implementation?
    // I just check against this program's stdin handle, and if they're the same, then the given
    // must be a tty for something... I guess...
    stream.as_raw_handle() == io::stdin().as_raw_handle()
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<impl Read> {
    Ok(io::stdin())
}
