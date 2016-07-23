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
