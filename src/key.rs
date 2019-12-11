use pancurses::Input;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Key {
    Char(char),
    Tab,
    Enter,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    Unknown,
}

impl From<Input> for Key {
    fn from(input: Input) -> Self {
        match input {
            Input::Character('\n') => Self::Enter,
            Input::Character('\t') => Self::Tab,
            Input::Character(ch) => Self::Char(ch),
            Input::KeyBackspace => Self::Backspace,
            Input::KeyLeft => Self::Left,
            Input::KeyRight => Self::Right,
            Input::KeyUp => Self::Up,
            Input::KeyDown => Self::Down,
            Input::KeyHome => Self::Home,
            Input::KeyEnd => Self::End,
            Input::Unknown(_) => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}
