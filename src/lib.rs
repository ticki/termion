#[warn(missing_docs)]

#[cfg(not(target_os = "redox"))]
extern crate libc;

#[cfg(not(target_os = "redox"))]
mod termios;

mod control;
pub use control::WriteExt;

mod input;
pub use input::ReadExt;

mod error;
pub use error::TerminalError;

mod raw;
pub use raw::{IntoRawMode, TerminalRestorer};

mod size;
pub use size::terminal_size;

mod color;
pub use color::Color;

mod style;
pub use style::Style;
