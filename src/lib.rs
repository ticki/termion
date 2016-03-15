//! Libterm is a pure Rust library for reading, manipulating, and handling terminals.
//!
//! This crate is not stable, yet. However, if you do want stability, you should specify the
//! revision (commit hash) in your `Cargo.toml`, this way builds are complete reproducible, and won't
//! break.

#![cfg_attr(feature = "nightly",
            feature(io))]

#![warn(missing_docs)]


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

mod color;
pub use color::Color;

mod style;
pub use style::Style;
