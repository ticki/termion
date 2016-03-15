extern crate termion;

use termion::{TermWrite, IntoRawMode, async_stdin};
use std::io::{Read, Write, stdout, stdin};
use std::thread;
use std::time::Duration;

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    stdout.clear().unwrap();
    stdout.goto(0, 0).unwrap();

    loop {
        stdout.clear_line().unwrap();

        let b = stdin.next();
        write!(stdout, "\r{:?}    <- This demonstrates the async read input char. Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", b).unwrap();
        if let Some(Ok(b'q')) = b {
            break;
        }

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(50));
        stdout.write(b"# ").unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));
        stdout.write(b"\r #").unwrap();
        stdout.goto(0, 0).unwrap();
        stdout.flush().unwrap();
    }
}
