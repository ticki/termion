extern crate termion;

use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    stdout.write(b"password: ").unwrap();
    stdout.flush().unwrap();

    let pass = stdin.read_passwd(&mut stdout);

    if let Ok(Some(pass)) = pass {
        stdout.write(pass.as_bytes()).unwrap();
        stdout.write(b"\n").unwrap();
    } else {
        stdout.write(b"Error\n").unwrap();
    }
}
