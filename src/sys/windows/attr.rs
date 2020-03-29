use std::{io, mem};

use super::crossterm_winapi::{ConsoleMode, Handle};
use super::Termios;

pub fn get_terminal_attr() -> io::Result<Termios> {
    let console_mode = ConsoleMode::from(Handle::current_in_handle()?);

    let mode = console_mode.mode()?;

    Ok(Termios(mode))
}

pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
    let console_mode = ConsoleMode::from(Handle::current_in_handle()?);

    console_mode.set_mode(termios.0)?;

    Ok(())
}

pub fn raw_terminal_attr(termios: &mut Termios) {
    // These are copied from the MSDocs.
    // Yes, technically, not the best, but Windows won't change these for obvious reasons.
    // We could link in winapi explicitly, as crossterm_winapi is already doing that, but
    // I feel it just adds a bit too much cruft, when we can just do this.
    //
    // https://docs.microsoft.com/en-us/windows/console/setconsolemode#parameters
    const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
    const ENABLE_LINE_INPUT: u32 = 0x0002;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;
    const RAW_MODE_MASK: u32 = ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT;

    termios.0 = termios.0 & !RAW_MODE_MASK;
}
