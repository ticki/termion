extern crate termion;

use termion::{TermWrite, Color, Style};
use std::io::{self, Write};

const LINE_NUM_BG: Color = Color::Grayscale(3);
const LINE_NUM_FG: Color = Color::Grayscale(18);
const ERROR_FG: Color = Color::Grayscale(17);
const INFO_LINE: &'static str = "|  ";

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.color(Color::LightGreen).unwrap();
    stdout.write("-- src/test/ui/borrow-errors.rs at 82:18 --\n".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(Color::Red).unwrap();
    stdout.style(Style::Bold).unwrap();
    stdout.write(b"error: ").unwrap();
    stdout.reset().unwrap();

    stdout.style(Style::Bold).unwrap();
    stdout.write(b"two closures require unique access to `vec` at the same time").unwrap();
    stdout.reset().unwrap();

    stdout.style(Style::Bold).unwrap();
    stdout.color(Color::Magenta).unwrap();
    stdout.write(b" [E0524]\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(b"79 ").unwrap();
    stdout.reset().unwrap();

    stdout.write(b"     let append = |e| {\n").unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(INFO_LINE.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(Color::Red).unwrap();
    stdout.write("                  ^^^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(ERROR_FG).unwrap();
    stdout.write(b"first closure is constructed here\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(b"80 ").unwrap();
    stdout.reset().unwrap();

    stdout.write(b"         vec.push(e)\n").unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(INFO_LINE.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(Color::Red).unwrap();
    stdout.write("         ^^^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(ERROR_FG).unwrap();
    stdout.write(b"previous borrow occurs due to use of `vec` in closure\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(b"81 ").unwrap();
    stdout.reset().unwrap();
    stdout.write(b"     };\n").unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(b"82 ").unwrap();
    stdout.reset().unwrap();
    stdout.write(b"     let append = |e| {\n").unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(INFO_LINE.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(Color::Red).unwrap();
    stdout.write("                  ^^^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(ERROR_FG).unwrap();
    stdout.write(b"second closure is constructed here\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(b"83 ").unwrap();
    stdout.reset().unwrap();

    stdout.write(b"         vec.push(e)\n").unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(INFO_LINE.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(Color::Red).unwrap();
    stdout.write("         ^^^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(ERROR_FG).unwrap();
    stdout.write(b"borrow occurs due to use of `vec` in closure\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(b"84 ").unwrap();
    stdout.reset().unwrap();
    stdout.write(b"     };\n").unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(b"85 ").unwrap();
    stdout.reset().unwrap();
    stdout.write(b" }\n").unwrap();

    stdout.color(LINE_NUM_FG).unwrap();
    stdout.bg_color(LINE_NUM_BG).unwrap();
    stdout.write(INFO_LINE.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(Color::Red).unwrap();
    stdout.write(" ^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(ERROR_FG).unwrap();
    stdout.write(b"borrow from first closure ends here\n").unwrap();
    stdout.reset().unwrap();
}
