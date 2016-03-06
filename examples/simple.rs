extern crate libterm;

use libterm::{TermControl, raw_mode};
use std::io::{Read, Write, stdout, stdin};

fn main() {
    let raw = raw_mode();
    let mut stdout = stdout();
    let mut stdin = stdin();

    stdout.goto(5, 5);
    stdout.clear();
    stdout.write(b"yo, 'q' will exit.");
    stdout.flush();

    let mut bytes = stdin.bytes();
    loop {
        let b = bytes.next().unwrap().unwrap();

        match b {
            b'q' => return,
            b'c' => stdout.clear(),
            a => stdout.write(&[a]),
        };

        stdout.flush();
    }
}
