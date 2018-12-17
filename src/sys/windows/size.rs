use std::{io, mem};

use super::winapi::um::{processenv, winbase, wincon};

/// Get the size of the terminal.
pub fn terminal_size() -> io::Result<(u16, u16)> {
    unsafe {
        let output_handle = processenv::GetStdHandle(winbase::STD_OUTPUT_HANDLE);
        let mut screen_info: wincon::CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
        wincon::GetConsoleScreenBufferInfo(output_handle, &mut screen_info);

        let columns = screen_info.srWindow.Right - screen_info.srWindow.Left + 1;
        let rows = screen_info.srWindow.Bottom - screen_info.srWindow.Top + 1;
        Ok((columns as u16, rows as u16))
    }
}
