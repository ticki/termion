libterm
=======

A pure Rust library for handling, manipulating and reading information about terminals. This provides a full-featured alternative to Termbox.

Supports Redox and POSIX. Untested on Windows.

A note on stability
-------------------

This crate is not stable, yet. However, if you do want stability, you should specify the revision (commit hash) in your `Cargo.toml`, this way builds are complete reproducible, and won't break.

Features
--------

- Raw mode.
- Cursor movement.
- Color output.
- Text formatting.
- Console size.
- Control sequences.
- Termios control.
- Password input.
- Redox support.
- 256-color mode.
- Panic-free error handling.
- Special keys events (modifiers, special keys, etc.).
- Asynchronous key events.

and much more.

Usage
-----

See `examples/`, and the documentation, which can be rendered using `cargo doc`.

For a more complete example, see [a minesweeper implementation](https://github.com/redox-os/games-for-redox/blob/master/src/minesweeper.rs), that I made for Redox using libterm.

TODO
----

- Mouse input

License
-------

MIT.
