extern crate termion;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
            .unwrap();

    let mut buf = String::new();
    'a: loop {
        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        write!(stdout, "\r{}    <- This demonstrates the async read input char. \
               Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", buf).unwrap();

        while let Some(next) = stdin.next() { match next {
            Ok(b'q') => break 'a,

            Ok(c) => buf.push(c as char),

            Err(e) => panic!("error: {:?}", e),
        }}

        stdout.flush().unwrap();

        //thread::sleep(Duration::from_millis(200));
        //stdout.write_all(b"# ").unwrap();
        //stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(100));
        //stdout.write_all(b"\r #").unwrap();
        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        stdout.flush().unwrap();
    }
}
