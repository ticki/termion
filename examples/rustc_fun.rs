extern crate termion;

use termion::{TermWrite, color, Style};
use std::io::{self, Write};

fn main() {
    let line_num_bg: color::AnsiValue = color::grayscale(3);
    let line_num_fg: color::AnsiValue = color::grayscale(18);
    let error_fg: color::AnsiValue = color::grayscale(17);
    let info_line: &'static str = "|  ";

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.color(color::LightGreen).unwrap();
    stdout.write("-- src/test/ui/borrow-errors.rs at 82:18 --\n".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(color::Red).unwrap();
    stdout.style(Style::Bold).unwrap();
    stdout.write(b"error: ").unwrap();
    stdout.reset().unwrap();

    stdout.style(Style::Bold).unwrap();
    stdout.write(b"two closures require unique access to `vec` at the same time").unwrap();
    stdout.reset().unwrap();

    stdout.style(Style::Bold).unwrap();
    stdout.color(color::Magenta).unwrap();
    stdout.write(b" [E0524]\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(b"79 ").unwrap();
    stdout.reset().unwrap();

    stdout.write(b"     let append = |e| {\n").unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(info_line.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(color::Red).unwrap();
    stdout.write("                  ^^^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(error_fg).unwrap();
    stdout.write(b"first closure is constructed here\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(b"80 ").unwrap();
    stdout.reset().unwrap();

    stdout.write(b"         vec.push(e)\n").unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(info_line.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(color::Red).unwrap();
    stdout.write("         ^^^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(error_fg).unwrap();
    stdout.write(b"previous borrow occurs due to use of `vec` in closure\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(b"81 ").unwrap();
    stdout.reset().unwrap();
    stdout.write(b"     };\n").unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(b"82 ").unwrap();
    stdout.reset().unwrap();
    stdout.write(b"     let append = |e| {\n").unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(info_line.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(color::Red).unwrap();
    stdout.write("                  ^^^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(error_fg).unwrap();
    stdout.write(b"second closure is constructed here\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(b"83 ").unwrap();
    stdout.reset().unwrap();

    stdout.write(b"         vec.push(e)\n").unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(info_line.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(color::Red).unwrap();
    stdout.write("         ^^^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(error_fg).unwrap();
    stdout.write(b"borrow occurs due to use of `vec` in closure\n").unwrap();
    stdout.reset().unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(b"84 ").unwrap();
    stdout.reset().unwrap();
    stdout.write(b"     };\n").unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(b"85 ").unwrap();
    stdout.reset().unwrap();
    stdout.write(b" }\n").unwrap();

    stdout.color(line_num_fg).unwrap();
    stdout.bg_color(line_num_bg).unwrap();
    stdout.write(info_line.as_bytes()).unwrap();
    stdout.reset().unwrap();
    stdout.color(color::Red).unwrap();
    stdout.write(" ^ ".as_bytes()).unwrap();
    stdout.reset().unwrap();

    stdout.color(error_fg).unwrap();
    stdout.write(b"borrow from first closure ends here\n").unwrap();
    stdout.reset().unwrap();
}
