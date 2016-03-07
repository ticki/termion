#![feature(io)]
#![feature(libc)]

#[warn(missing_docs)]

extern crate libc;

mod termios;

mod control;
pub use control::TermControl;

mod raw;
pub use raw::{raw_mode, TerminalRestorer};

mod size;
pub use size::termsize;

mod color;
pub use color::Color;

mod mode;
pub use mode::Mode;

mod extra;
pub use extra::ReadExt;
