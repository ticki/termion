extern crate winapi;

use self::winapi::shared::minwindef::DWORD;

#[derive(Clone, Copy, Debug)]
pub struct Termios(DWORD, DWORD);

pub mod attr;
pub mod size;
pub mod tty;