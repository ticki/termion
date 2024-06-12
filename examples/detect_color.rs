extern crate termion;

use std::io::stdout;
use termion::color::{AnsiValue, Bg, DetectColors};
use termion::raw::IntoRawMode;

fn main() {
    let count;
    {
        let mut term = stdout().into_raw_mode().unwrap();
        count = term.available_colors().unwrap();
    }

    println!("This terminal supports {} colors.", count);
    for i in 0..count {
        print!("{} {}", Bg(AnsiValue(i as u8)), Bg(AnsiValue(0)));
    }
    println!();
}
