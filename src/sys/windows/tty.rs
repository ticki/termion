use std::io::{self, Read, Result, Stdin, Stdout, Write};
use std::os::windows::io::AsRawHandle;
extern crate crossterm;
use self::crossterm::tty::IsTty as _;

/// Is this stream a TTY?
pub fn is_tty<T: AsRawHandle>(stream: &T) -> bool {
    stream.is_tty()
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<impl Read + Write> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    Ok(TerminalHandle {
        stdin,
        stdout,
    })
}

struct TerminalHandle {
    stdin: Stdin,
    stdout: Stdout,
}

impl Read for TerminalHandle {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.stdin.read(buf)
    }
}

impl Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }
}
