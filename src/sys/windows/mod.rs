extern crate crossterm_winapi;

#[derive(Clone, Copy, Debug)]
pub struct Termios{inp:u32, out:u32}

pub mod attr;
pub mod size;
pub mod tty;