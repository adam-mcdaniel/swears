// extern crate termion;
// use std::io::{stderr, stdin, stdout, Write};
// use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;

pub mod key;
pub use key::Key;
pub mod color;
pub use color::{Color, ColorString, Colorize};
pub mod terminal;
pub use terminal::Terminal;
