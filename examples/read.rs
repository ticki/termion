extern crate libterm;

use libterm::ReadExt;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut stdin = stdin();

    stdout.write(b"password: ").unwrap();
    stdout.flush().unwrap();

    let pass = stdin.read_passwd();

    if let Some(pass) = pass {
        stdout.write(pass.as_bytes()).unwrap();
        stdout.write(b"\n").unwrap();
    } else {
        stdout.write(b"Error\n").unwrap();
    }
}
