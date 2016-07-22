extern crate termion;

use termion::{TermWrite, color, Style};

use std::io;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.color(color::Red).unwrap();
    println!("Red");

    stdout.color(color::Blue).unwrap();
    println!("Blue");

    stdout.style(Style::Bold).unwrap();
    println!("Blue'n'Bold");

    stdout.color(color::true_rgb(255, 100, 0)).unwrap();
    println!("TRUECOLOR");

    stdout.reset().unwrap();
    stdout.style(Style::Italic).unwrap();
    println!("Just plain italic")
}
