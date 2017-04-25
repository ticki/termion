extern crate termion;

use termion::raw::IntoRawMode;
use termion::{input, color};

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    
    loop{
        if let Some(input) = input::highlighted_read_line(&mut stdin, &mut stdout, &format!("{} > ", std::env::current_dir().unwrap().to_str().unwrap()), |line|{
            let parts = line.split(' ');
            let num_parts = line.split(' ').count();
            let mut result = String::with_capacity(line.len() + num_parts * 5);
            for part in parts{
                if part.len() == 0 {
                    result.push(' ');
                    continue;
                }
                
                let color = match part.chars().next().unwrap(){
                    'a' ... 'z' | 'A' ... 'Z' => format!("{}", color::Fg(color::Yellow)),
                    '-' => format!("{}", color::Fg(color::Green)),
                    _ => format!("{}", color::Fg(color::Red))
                };
                
                result.push_str(&color);
                result.push_str(part);
                result.push_str(&format!("{}", color::Fg(color::Reset)));
                result.push(' ');
            }
            result.pop();
            result
        }).unwrap(){
            println!("\n\r{}", input);
        }else{
            println!("\n\r");
            break;
        }
    }
}
