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

#[cfg(not(target_os = "redox"))]
extern crate libc;

#[cfg(not(target_os = "redox"))]
mod termios;

#[cfg(target_os = "redox")]
extern crate redox_termios;

#[cfg(target_os = "redox")]
extern crate syscall;

mod async;
pub use async::{AsyncReader, async_stdin};

mod error;

mod size;
pub use size::terminal_size;

mod tty;
pub use tty::{is_tty, get_tty};

#[macro_use]
mod macros;
pub mod clear;
pub mod color;
pub mod cursor;
pub mod event;
pub mod input;
pub mod raw;
pub mod screen;
pub mod scroll;
pub mod style;
mod utils;
