extern crate termion;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}q to exit. Type stuff, use alt, click around...", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

    let mut x = 1;
    let mut y = 1;

    for c in stdin.events() {
        let evt = c.unwrap();
        writeln!(stdout, "{:?}{}{}", evt, termion::cursor::Goto(5, 5), termion::clear::CurrentLine).unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, a, b) |
                    MouseEvent::Release(a, b) => {
                        x = a;
                        y = b;
                    }
                }
            }
            _ => {}
        }
        writeln!(stdout, "{:?}{}", evt, termion::cursor::Goto(x, y)).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
