extern crate termion;

use termion::{color, style};

fn named_colors() {
    struct Col{name: &'static str, color: &'static dyn color::Color};
    macro_rules! C {
        ($name:ident) => {
            Col{name: stringify!($name), color: &color::$name}
        };
    }
    let cols = [
        C!(Black),
        C!(Red),
        C!(Green),
        C!(Yellow),
        C!(Blue),
        C!(Magenta),
        C!(Cyan),
        C!(White),
        C!(LightBlack),
        C!(LightRed),
        C!(LightGreen),
        C!(LightYellow),
        C!(LightBlue),
        C!(LightMagenta),
        C!(LightCyan),
        C!(LightWhite),
    ];
    fn show_fg_bg(fgs: &[Col], bgs: &[Col]) {
        print!("{:12} ", "");
        for bg in bgs {
            print!("{:^12} ", bg.name);
        }
        println!();

        for fg in fgs {
            print!("{:12} ", fg.name);
            for bg in bgs {
                print!("{}{}{:^12}{}{} ",
                    color::Fg(fg.color), color::Bg(bg.color),
                    "XXX",
                    color::Fg(color::Reset), color::Bg(color::Reset)
                );
            }
            println!();
        }
        println!();
    }

    println!("Named colors");
    println!("============");
    show_fg_bg(&cols, &cols[..cols.len()/2]);
    show_fg_bg(&cols, &cols[cols.len()/2..]);
}

fn six_by_six_by_six_colors() {
    println!("216 colors");
    println!("===========");
    let rgb_216 = (0..6).flat_map(
        move |r| (0..6).flat_map(
            move |g| (0..6).map(move |b| (r, g, b))
        )
    );
    for (idx, (r, g, b)) in rgb_216.enumerate() {
        if idx > 0 && idx % 36 == 0 {
            println!();
        }
        print!("{}XX{}{}",
            color::Fg(color::AnsiValue::rgb(r, g, b)),
            color::Fg(color::Reset), color::Bg(color::Reset));
    }
    println!();
    println!();
}

fn grayscale_colors() {
    fn show_on_bg(bg: &dyn color::Color) {
        for g in 0..24 {
            print!("{}{}XXX{}{}",
                color::Fg(color::AnsiValue::grayscale(g)), color::Bg(bg),
                color::Fg(color::Reset), color::Bg(color::Reset)
            );
        }
        println!();
    }
    println!("Grayscale colors");
    println!("================");
    show_on_bg(&color::Black);
    show_on_bg(&color::White);
    println!();
}

fn true_color() {
    println!("Truecolor");
    println!("=========");
    fn show_rgb(cols: impl Iterator<Item=(u8,u8,u8)>, columns: usize) {
        for (idx, (r, g, b)) in cols.enumerate() {
            if idx > 0 && idx % columns == 0 {
                println!();
            }
            print!("{}XX{}{}",
                color::Fg(color::Rgb(r, g, b)),
                color::Fg(color::Reset), color::Bg(color::Reset)
            );
        }
        println!();
    }
    // reproduce 216 color palette with 8-bit RGB values
    let rgb_216 = (0..6).flat_map(
        move |r| (0..6).flat_map(
            move |g| (0..6).map(
                move |b| (
                    (128*r as u16/3) as u8,
                    (128*g as u16/3) as u8,
                    (128*b as u16/3) as u8
                )
            )
        )
    );
    println!("216 colors");
    println!("----------");
    show_rgb(rgb_216, 36);
    println!();

    println!("reds");
    println!("====");
    show_rgb((0..=255).map(|r|(r, 0, 0)), 32);
    println!();

    println!("greens");
    println!("======");
    show_rgb((0..=255).map(|g|(0, g, 0)), 32);
    println!();

    println!("blues");
    println!("=====");
    show_rgb((0..=255).map(|b|(0, 0, b)), 32);
    println!();
}

fn styles() {
    struct Style{name: &'static str, style: &'static dyn std::fmt::Display};
    macro_rules! S {
        ($name:ident) => {
            Style{name: stringify!($name), style: &style::$name}
        };
    }
    let styles = [
        S!(Bold),
        S!(Faint),
        S!(Italic),
        S!(Underline),
        S!(Blink),
        S!(Invert),
        S!(CrossedOut),
        S!(Framed),
    ];
    println!("Styles");
    println!("======");
    for s in &styles {
        println!("{:12} {}XXX{} {}{}XXX{} {}{}XXX{} {}{}XXX{}",
            s.name,
            s.style, style::Reset,
            s.style, color::Fg(color::Red), style::Reset,
            s.style, color::Fg(color::Green), style::Reset,
            s.style, color::Fg(color::Blue), style::Reset,
        );
    }
    println!();
}

fn main() {
    named_colors();
    six_by_six_by_six_colors();
    grayscale_colors();
    true_color();

    styles();
}
