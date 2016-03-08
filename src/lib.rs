#![feature(io)]
#![feature(libc)]

#[warn(missing_docs)]

#[cfg(not(target_os = "redox"))]
extern crate libc;

#[cfg(not(target_os = "redox"))]
mod termios;

mod control;
pub use control::TermControl;

mod error;
pub use error::TerminalError;

mod raw;
pub use raw::{IntoRawMode, TerminalRestorer};

// TODO Redox terminal size
#[cfg(not(target_os = "redox"))]
mod size;
#[cfg(not(target_os = "redox"))]
pub use size::terminal_size;

mod color;
pub use color::Color;

mod mode;
pub use mode::Mode;

mod extra;
pub use extra::ReadExt;
