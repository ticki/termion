extern crate crossterm_winapi;

#[derive(Clone, Copy, Debug)]
pub struct Termios(u32);

pub mod attr;
pub mod size;
pub mod tty;