extern crate termion;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    writeln!(stdout,
             "{}{}q to exit. Type stuff, use alt, click around...",
             termion::clear::All,
             termion::cursor::Goto(1, 1))
        .unwrap();

    let mut x = 5;
    let mut y = 5;

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, a, b) |
                    MouseEvent::Release(a, b) |
                    MouseEvent::Hold(a, b) => {
                        x = a;
                        y = b;
                    }
                }
            }
            _ => {}
        }
        write!(stdout,
               "{}{}  {:?}{}",
               termion::clear::All,
               termion::cursor::Goto(x, y),
               evt,
               termion::cursor::Goto(x, y))
            .unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
