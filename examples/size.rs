extern crate libterm;

use libterm::terminal_size;

fn main() {
    println!("Size is {:?}", terminal_size().unwrap())
}
