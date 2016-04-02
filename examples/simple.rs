extern crate termion;

use termion::{TermWrite, IntoRawMode, Color, Style};
use std::io::{Read, Write, stdout, stdin};

fn main() {
    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    // Move the cursor to (5, 5)
    stdout.goto(5, 5).unwrap();
    // Clear the screen.
    stdout.clear().unwrap();
    // Set style to bold.
    stdout.style(Style::Bold).unwrap();
    // Write some guiding stuff
    stdout.write(b"yo, 'q' will exit.").unwrap();
    // Reset the style.
    stdout.reset().unwrap();
    // Flush and goto (20, 10)
    stdout.flush().unwrap();
    stdout.goto(20, 10).unwrap();

    let mut bytes = stdin.bytes();
    loop {
        let b = bytes.next().unwrap().unwrap();

        match b {
            // Quit
            b'q' => return,
            // Clear the screen
            b'c' => stdout.clear(),
            // Set red color
            b'r' => stdout.color(Color::Rgb(5, 0, 0)),
            // Write it to stdout.
            a => stdout.write(&[a]),
        }.unwrap();

        stdout.flush().unwrap();
    }
}
