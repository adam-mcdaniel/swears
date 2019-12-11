use swears::{Colorize, Key, Terminal};

fn main() {
    let term = Terminal::with_color();
    term.disable_echo();
    term.enable_scrolling();

    term.println("Enter your password: ".green());
    let mut password = String::new();
    for key in &term {
        match key {
            Key::Char(ch) => password.push(ch),
            Key::Backspace => { password.pop(); },
            Key::Enter => break,
            _ => {}
        }
    }

    term.println(format!("Your password was '{}'", password).red());
    term.get_key();
}
