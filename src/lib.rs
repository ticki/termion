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
//! For more information refer to the [README](https://github.com/redox-os/termion).
#![warn(missing_docs)]

extern crate numtoa;
#[cfg(feature = "serde")]
extern crate serde;

#[cfg(target_os = "redox")]
#[path = "sys/redox/mod.rs"]
mod sys;

#[cfg(all(unix, not(target_os = "redox")))]
#[path = "sys/unix/mod.rs"]
mod sys;

pub use sys::size::terminal_size;
#[cfg(all(unix, not(target_os = "redox")))]
pub use sys::size::terminal_size_pixels;
pub use sys::tty::{get_tty, is_tty};

mod r#async;
pub use r#async::{async_stdin, AsyncReader};

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

#[cfg(test)]
mod test {
    use std::os::fd::AsFd;

    use super::sys;

    #[test]
    fn test_get_terminal_attr() {
        let stdout = std::io::stdout();
        sys::attr::get_terminal_attr(stdout.as_fd()).unwrap();
        sys::attr::get_terminal_attr(stdout.as_fd()).unwrap();
        sys::attr::get_terminal_attr(stdout.as_fd()).unwrap();
    }

    #[test]
    fn test_set_terminal_attr() {
        let stdout = std::io::stdout();
        let ios = sys::attr::get_terminal_attr(stdout.as_fd()).unwrap();
        sys::attr::set_terminal_attr(stdout.as_fd(), &ios).unwrap();
    }

    #[test]
    fn test_size() {
        sys::size::terminal_size().unwrap();
    }
}
