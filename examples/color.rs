extern crate termion;

use termion::{color::{self, Fg}, style};

fn main() {
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
    println!("{}Just plain italic{}", style::Italic, style::Reset);

    println!("{}", style::Reset);

    println!("{}Yellow{}", color::Fg(color::Yellow), Fg(color::Reset));
    println!("{}LightYellow{}", color::Fg(color::LightYellow), Fg(color::Reset));
}
