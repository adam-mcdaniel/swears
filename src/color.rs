use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Yellow,
    Magenta,
    Default,
    Cyan,
    Black,
    White,
    Red,
    Blue,
    Green,
}

#[derive(Clone, Debug)]
pub struct ColorString(pub Color, pub String);

impl ColorString {
    pub fn to_color(&self) -> Color {
        let Self(color, _) = self;
        *color
    }
}

impl ToString for ColorString {
    fn to_string(&self) -> String {
        let Self(_, s) = self;
        s.clone()
    }
}

impl Default for ColorString {
    fn default() -> Self {
        Self(Color::Default, String::new())
    }
}

impl<T: Display> From<T> for ColorString {
    fn from(t: T) -> Self {
        Self(Color::Default, t.to_string())
    }
}

pub trait Colorize {
    fn red(self) -> ColorString;
    fn blue(self) -> ColorString;
    fn green(self) -> ColorString;
    fn yellow(self) -> ColorString;
    fn magenta(self) -> ColorString;
    fn cyan(self) -> ColorString;
    fn black(self) -> ColorString;
    fn white(self) -> ColorString;
    fn default(self) -> ColorString;
}

impl<T: Display> Colorize for T {
    fn red(self) -> ColorString {
        ColorString(Color::Red, self.to_string())
    }

    fn blue(self) -> ColorString {
        ColorString(Color::Blue, self.to_string())
    }

    fn green(self) -> ColorString {
        ColorString(Color::Green, self.to_string())
    }

    fn yellow(self) -> ColorString {
        ColorString(Color::Yellow, self.to_string())
    }

    fn magenta(self) -> ColorString {
        ColorString(Color::Magenta, self.to_string())
    }

    fn cyan(self) -> ColorString {
        ColorString(Color::Cyan, self.to_string())
    }

    fn black(self) -> ColorString {
        ColorString(Color::Black, self.to_string())
    }

    fn white(self) -> ColorString {
        ColorString(Color::White, self.to_string())
    }

    fn default(self) -> ColorString {
        ColorString(Color::Default, self.to_string())
    }
}
