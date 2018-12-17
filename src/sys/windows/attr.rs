use std::{io, mem};

use super::{DWORD, Termios};
use super::winapi::um::{consoleapi, handleapi, processenv, winbase, wincon};

pub fn get_terminal_attr() -> io::Result<Termios> {
    let input_mode = get_console_mode(winbase::STD_INPUT_HANDLE)?;
    let output_mode = get_console_mode(winbase::STD_OUTPUT_HANDLE)?;

    Ok(Termios(input_mode, output_mode))
}

pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
    set_console_mode(winbase::STD_INPUT_HANDLE, termios.0)?;
    set_console_mode(winbase::STD_OUTPUT_HANDLE, termios.1)?;
    Ok(())
}

fn get_console_mode(handle: DWORD) -> io::Result<DWORD> {
    unsafe {
        let handle = processenv::GetStdHandle(handle);
        if handle == handleapi::INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }
        let mut mode: DWORD = mem::zeroed();
        consoleapi::GetConsoleMode(handle, &mut mode);
        Ok(mode)
    }
}

fn set_console_mode(handle: DWORD, mode: DWORD) -> io::Result<()> {
    unsafe {
        let handle = processenv::GetStdHandle(handle);
        if handle == handleapi::INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }
        let check = consoleapi::SetConsoleMode(handle, mode);
        if check != 1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

pub fn raw_terminal_attr(termios: &mut Termios) {
    termios.0 = wincon::ENABLE_VIRTUAL_TERMINAL_INPUT;
    termios.1 = wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING
        | wincon::ENABLE_PROCESSED_OUTPUT
        | wincon::DISABLE_NEWLINE_AUTO_RETURN;
}
