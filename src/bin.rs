use swears::{Colorize, Key, Terminal};

fn main() {
    let term = Terminal::new();
    term.disable_echo();
    term.enable_scrolling();

    term.print("Enter your password: ".green());
    let mut password = String::new();
    for key in &term {
        match key {
            Key::Char(ch) => password.push(ch),
            Key::Backspace => { password.pop(); },
            Key::Enter => break,
            _ => {}
        }
    }
    drop(term);
    println!("Your password was '{}'", password);
}
