use swears::{Colorize, Terminal};

fn main() {
    let term = Terminal::with_color();
    term.disable_echo();
    term.enable_scrolling();

    for n in 0..100 {
        term.println(format!("Testing: {}", n).red());
    }
    
    term.println("Press any key to exit");
    term.get_key();
}
