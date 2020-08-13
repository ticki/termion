extern crate termion;

use termion::event::*;
use termion::cursor::{self, DetectCursorPos};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    writeln!(stdout, "{}", termion::clear::All).unwrap();
    fn msg(out: &mut impl Write) {
        writeln!(out, "{}q to exit. Type stuff, use alt, click around...{}",
           termion::cursor::Goto(1, 1), termion::clear::UntilNewline).unwrap();
    }
    let (mut x, mut y) = (1, 2);
    msg(&mut stdout);
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, a, b) |
                    MouseEvent::Release(a, b) |
                    MouseEvent::Hold(a, b) => {
                        write!(stdout, "{}", cursor::Goto(a, b)).unwrap();
                        writeln!(stdout, "{}{}Cursor is at: ({},{}), event {:?}",
                               cursor::Goto(x, y),
                               termion::clear::UntilNewline,
                               a,
                               b,
                               me
                        ).unwrap();
                        let (x1, y1) = stdout.cursor_pos().unwrap();
                        x = x1; y = y1;
                        msg(&mut stdout);
                        write!(stdout, "{}", cursor::Goto(a, b)).unwrap();
                    }
                }
            }
            _ => {}
        }

        stdout.flush().unwrap();
    }
}
