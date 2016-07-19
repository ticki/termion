extern crate termion;

fn main() {
    use termion::{TermRead, TermWrite, IntoRawMode, Key, Event, MouseEvent};
    use std::io::{Write, stdout, stdin};

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap().with_mouse().unwrap();

    stdout.clear().unwrap();
    stdout.goto(0, 0).unwrap();
    stdout.write(b"q to exit. Type stuff, use alt, click around...").unwrap();
    stdout.flush().unwrap();

    let mut x = 0;
    let mut y = 0;

    for c in stdin.events() {
        stdout.goto(5, 5).unwrap();
        stdout.clear_line().unwrap();
        let evt = c.unwrap();
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
        println!("{:?}", evt);
        stdout.goto(x, y).unwrap();
        stdout.flush().unwrap();
    }

    stdout.show_cursor().unwrap();
}
