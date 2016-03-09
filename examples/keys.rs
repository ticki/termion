extern crate libterm;

use libterm::{TermRead, TermWrite, IntoRawMode, Color, Style, Key};
use std::io::{Read, Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    stdout.clear();
    stdout.goto(0, 0);
    stdout.write(b"q to exit. Type stuff, use alt, and so on.");
    stdout.hide_cursor();
    stdout.flush();

    for c in stdin.keys() {
        stdout.goto(5, 5);
        stdout.clear_line();
        match c {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("∆"),
            Key::Down => println!("∇"),
            Key::Backspace => println!("×"),
            Key::Invalid => println!("???"),
            Key::Error => println!("ERROR"),
        }
        stdout.flush();
    }

    stdout.show_cursor();
}
