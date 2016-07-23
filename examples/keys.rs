extern crate termion;

use termion::input::{TermRead, Key};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();

    for c in stdin.keys() {
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => writeln!(stdout, "{}", c).unwrap(),
            Key::Alt(c) => writeln!(stdout, "^{}", c).unwrap(),
            Key::Ctrl(c) => writeln!(stdout, "*{}", c).unwrap(),
            Key::Left => writeln!(stdout, "←").unwrap(),
            Key::Right => writeln!(stdout, "→").unwrap(),
            Key::Up => writeln!(stdout, "↑").unwrap(),
            Key::Down => writeln!(stdout, "↓").unwrap(),
            Key::Backspace => writeln!(stdout, "×").unwrap(),
            Key::Invalid => writeln!(stdout, "???").unwrap(),
            _ => {},
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
