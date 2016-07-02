//! Termion is a pure Rust, bindless library for low-level handling, manipulating
//! and reading information about terminals. This provides a full-featured
//! alternative to Termbox.
//!
//! Termion aims to be simple and yet expressive. It is bindless, meaning that it
//! is not a front-end to some other library (e.g., ncurses or termbox), but a
//! standalone library directly talking to the TTY.
//!
//! Supports Redox, Mac OS X, and Linux (or, in general, ANSI terminals).
//!
//! For more information refer to the [README](https://github.com/ticki/termion).
#![warn(missing_docs)]

#![cfg_attr(feature = "nightly", feature(io))]


#[cfg(not(target_os = "redox"))]
extern crate libc;

#[cfg(not(target_os = "redox"))]
mod termios;

mod control;
pub use control::TermWrite;

mod async;
pub use async::{AsyncReader, async_stdin};

mod input;
pub use input::{TermRead, Key};
#[cfg(feature = "nightly")]
pub use input::Keys;

mod raw;
pub use raw::{IntoRawMode, RawTerminal};

mod size;
pub use size::terminal_size;

/// ANSI colors.
pub mod color;

/// Deprecated reexport.
#[deprecated]
pub use color::Palette as Color;

mod style;
pub use style::Style;
