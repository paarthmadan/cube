extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::{
    io::{Write, stdin, stdout},
    process,
};

fn main() {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout,
               "{}{}q to exit. Type stuff, use alt, and so on.{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               termion::cursor::Hide)
                .unwrap();
        stdout.flush().unwrap();
        for evt in stdin.keys() {
            write!(stdout,
                   "{}{}",
                   termion::cursor::Goto(1, 1),
                   termion::clear::CurrentLine)
                    .unwrap();
            match evt {
                Ok(key) => {
                    match key {
                        Key::Char('q') => process::exit(1),
                        Key::Char(c) => println!("{}", c),
                        Key::Alt(c) => println!("^{}", c),
                        Key::Ctrl(c) => println!("*{}", c),
                        Key::Esc => println!("ESC"),
                        Key::Left => println!("←"),
                        Key::Right => println!("→"),
                        Key::Up => println!("↑"),
                        Key::Down => println!("↓"),
                        Key::Backspace => println!("×"),
                        _ => {}
                    }
                    stdout.flush().unwrap();
                }
                Err(_) => println!("Error"),
            }
        }
        write!(stdout, "{}", termion::cursor::Show).unwrap();
}
