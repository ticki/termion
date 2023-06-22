extern crate termion;

#[cfg(not(windows))]
use std::fs;

#[cfg(windows)]
use std::io;

fn main() {
    #[cfg(not(windows))]
    let stream = fs::File::create("/dev/stdout").unwrap();
    #[cfg(windows)]
    let stream = io::stdin();

    if termion::is_tty(&stream) {
        println!("This is a TTY!");
    } else {
        println!("This is not a TTY :(");
    }
}
