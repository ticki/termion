# 4.0.3

Remove unused code and update dependencies.

# 4.0.2

Fixes an error check in Ctrl-Arrow code, no difference in behavior. Cleaned up examples.

# 4.0.1

Fixes a regression in function keys F5 and above.

# 4.0.0

4.0.0 adds support for horizontal scrolling when working with `MouseTerminal`.

## 3.0.0 to 4.0.0 guide

A change is only necessary if you were matching on all variants of the `MouseEvent` enum without a wildcard.
In this case, you need to either handle the two new variants, `MouseLeft` and `MouseRight`, or add a wildcard.

# 3.0.0

v3 release improves `raw` terminal API and enables support of any TTY target.

## 2.0.0 to 3.0.0 guide

Changes are only required if you were using `IntoRawMode` on generic terminals `W: Write`. Now, terminal
is also required to implement [`AsFd` trait][AsFd-trait]. So replacing generic bounds with `W: Write + AsFd`
should be sufficient.

[AsFd-trait]: https://doc.rust-lang.org/std/os/fd/trait.AsFd.html

# 1.0.0

Termion 1.0.0 is out! This release is breaking, which is also the reason for the semver bump.

## Highlights

Lot'ta goodies.

- **Mouse support:** If you enabled mouse mode through the `MouseTerminal` struct, you can get mouse events (thanks to IGI-111).
- **TrueColor support:** You can now use true color, by the `Rgb` struct.
- **A complete revision of the way escapes are handled:** Everything is now done through `Display` instead of custom traits.
- **`isatty` wrapper:** `termion::is_tty` takes any `T: AsRawFd` and gives you a `bool`.
- **Crates.io release:** Previously, it was distributed solely through git. This turns out to be very convinient, but quite critical whenever making breaking changes (that is, major semver bumps).

## 0.1.0 to 1.0.0 guide

This sample table gives an idea of how to go bu converting to the new major
version of Termion.

| 0.1.0                          | 1.0.0
|--------------------------------|---------------------------
| `use termion::IntoRawMode`     | `use termion::raw::IntoRawMode`
| `stdout.color(color::Red);`    | `write!(stdout, "{}", color::Fg(color::Red));`
| `stdout.color_bg(color::Red);` | `write!(stdout, "{}", color::Bg(color::Red));`
| `stdout.goto(x, y);`           | `write!(stdout, "{}", cursor::Goto(x, y));`
| `color::rgb(r, g, b);`         | `color::Rgb(r, g, b)` (truecolor)
| `x.with_mouse()`               | `MouseTerminal::from(x)`

## An example

```rust
#![feature(step_by)]

extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn rainbow<W: Write>(stdout: &mut W, blue: u8) {
    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();

    for red in (0..255).step_by(8 as u8) {
        for green in (0..255).step_by(4) {
            write!(stdout, "{} ", termion::color::Bg(termion::color::Rgb(red, green, blue))).unwrap();
        }
        write!(stdout, "\n\r").unwrap();
    }

    writeln!(stdout, "{}b = {}", termion::style::Reset, blue).unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    writeln!(stdout, "{}{}{}Use the arrow keys to change the blue in the rainbow.",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();

    let mut blue = 172u8;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => {
                blue = blue.saturating_add(4);
                rainbow(&mut stdout, blue);
            },
            Key::Down => {
                blue = blue.saturating_sub(4);
                rainbow(&mut stdout, blue);
            },
            Key::Char('q') => break,
            _ => {},
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
```
