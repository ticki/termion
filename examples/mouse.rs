extern crate termion;

use termion::event::*;
use termion::cursor;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    writeln!(stdout,
             "{}{}q to exit. Type stuff, use alt, click around...",
             termion::clear::All,
             termion::cursor::Goto(1, 1))
            .unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, a, b) |
                    MouseEvent::Release(_, a, b) |
                    MouseEvent::Hold(_, a, b) => {
                        write!(stdout,
                               "{}{}Cursor is {:?}{}",
                               cursor::Goto(5, 5),
                               termion::clear::UntilNewline,
                               me,
                               cursor::Goto(a, b))
                                .unwrap();
                    }
                }
            }
            _ => {}
        }

        stdout.flush().unwrap();
    }
}
