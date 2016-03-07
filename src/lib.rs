#![feature(libc)]
extern crate libc;

mod termios;

mod control;
pub use control::TermControl;

mod raw;
pub use raw::{raw_mode, TerminalRestorer};

mod size;
pub use size::termsize;
