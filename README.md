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

and much more.

TODO
----

- Mouse input

Usage
-----

See `examples/`.

License
-------

MIT.
