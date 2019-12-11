use crate::{Color, ColorString, Key};
use pancurses::{
    resize_term, curs_set, def_prog_mode, def_shell_mode, doupdate, echo, endwin, init_pair,
    initscr, noecho, noraw, raw, reset_prog_mode, reset_shell_mode, resize_term, start_color,
    Window, COLOR_BLACK, COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_PAIR, COLOR_RED,
    COLOR_WHITE, COLOR_YELLOW,
};

use std::i32::{MAX, MIN};


#[derive(Debug)]
pub struct Terminal {
    win: Window,
}

impl Terminal {
    pub fn new() -> Self {
        let result = Self { win: initscr() };
        raw();
        resize_term(24, 80);
        result.win.keypad(true);
        result.disable_echo();
        result.disable_scrolling();
        result.hide_cursor();
        result
    }

    pub fn with_color() -> Self {
        let result = Self::new();
        start_color();
        init_pair(COLOR_WHITE, COLOR_WHITE, COLOR_BLACK);
        init_pair(COLOR_RED, COLOR_RED, COLOR_BLACK);
        init_pair(COLOR_BLUE, COLOR_BLUE, COLOR_BLACK);
        init_pair(COLOR_GREEN, COLOR_GREEN, COLOR_BLACK);
        init_pair(COLOR_CYAN, COLOR_CYAN, COLOR_BLACK);
        init_pair(COLOR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
        init_pair(COLOR_MAGENTA, COLOR_MAGENTA, COLOR_BLACK);
        init_pair(COLOR_BLACK, COLOR_BLACK, COLOR_BLACK);
        result
    }

    pub fn hide_cursor(&self) {
        curs_set(0);
    }
    pub fn show_cursor(&self) {
        curs_set(1);
    }

    pub fn enable_echo(&self) {
        echo();
    }
    pub fn disable_echo(&self) {
        noecho();
    }

    pub fn enable_scrolling(&self) {
        resize_term(MAX, MAX);
        self.win.setscrreg(MIN, MAX);
        self.win.scrollok(true);
    }

    pub fn disable_scrolling(&self) {
        self.win.scrollok(false);
    }

    pub fn get_key(&self) -> Option<Key> {
        if let Some(input) = self.win.getch() {
            Some(Key::from(input))
        } else {
            None
        }
    }

    pub fn set_input_timeout(&self, milliseconds: u32) {
        self.win.timeout(milliseconds as i32);
    }

    pub fn pause(&self) {
        def_prog_mode();
        noraw();
        reset_shell_mode();
        endwin();
    }

    pub fn resume(&self) {
        def_shell_mode();
        raw();
        reset_prog_mode();
        self.win.refresh();
        doupdate();
    }

    pub fn refresh(&self) {
        self.win.touch();
        self.win.refresh();
    }

    pub fn clear(&self) {
        self.win.clear();
    }

    pub fn move_print(&self, mut x: i32, mut y: i32, s: impl Into<ColorString>) {
        let x_initial = x;
        let cs = s.into();
        let color_num = match cs.to_color() {
            Color::Red => COLOR_RED,
            Color::Green => COLOR_GREEN,
            Color::Blue => COLOR_BLUE,
            Color::Cyan => COLOR_CYAN,
            Color::Magenta => COLOR_MAGENTA,
            Color::Yellow => COLOR_YELLOW,
            Color::White => COLOR_WHITE,
            Color::Black => COLOR_BLACK,
            Color::Default => COLOR_WHITE,
        };

        self.win.attrset(COLOR_PAIR(color_num as u64));

        for ch in cs.to_string().chars() {
            if ch == '\n' {
                y += 1;
                x = x_initial;
            } else {
                x += 1;
                self.win.mvaddch(y, x, ch);
            }
        }
    }

    pub fn println(&self, s: impl Into<ColorString>) {
        let cs = s.into();
        let color = cs.to_color();
        self.print(ColorString(color, cs.to_string() + "\n\r"));
    }

    pub fn print(&self, s: impl Into<ColorString>) {
        let cs = s.into();
        let color_num = match cs.to_color() {
            Color::Red => COLOR_RED,
            Color::Green => COLOR_GREEN,
            Color::Blue => COLOR_BLUE,
            Color::Cyan => COLOR_CYAN,
            Color::Magenta => COLOR_MAGENTA,
            Color::Yellow => COLOR_YELLOW,
            Color::White => COLOR_WHITE,
            Color::Black => COLOR_BLACK,
            Color::Default => COLOR_WHITE,
        };

        self.win.attrset(COLOR_PAIR(color_num as u64));
        self.win.printw(cs.to_string());
    }

    pub fn clear_current_line(&self) {
        self.print("\r");
        self.win.clrtoeol();
    }

    pub fn move_cursor(&self, x: i32, y: i32) {
        self.win.mv(y, x);
    }

    pub fn get_size(&self) -> (i32, i32) {
        (self.get_width(), self.get_height())
    }

    pub fn get_width(&self) -> i32 {
        self.win.get_max_x()
    }

    pub fn get_height(&self) -> i32 {
        self.win.get_max_y()
    }

    pub fn get_cursor(&self) -> (i32, i32) {
        (self.get_cursor_x(), self.get_cursor_y())
    }

    pub fn get_cursor_x(&self) -> i32 {
        self.win.get_cur_x()
    }

    pub fn get_cursor_y(&self) -> i32 {
        self.win.get_cur_y()
    }

    pub fn quit(&self) {
        endwin();
    }
}

impl Iterator for Terminal {
    type Item = Key;
    fn next(&mut self) -> Option<Key> {
        self.get_key()
    }
}

impl Iterator for &Terminal {
    type Item = Key;
    fn next(&mut self) -> Option<Key> {
        self.get_key()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.quit();
    }
}

impl Clone for Terminal {
    fn clone(&self) -> Self {
        let win = self.win.dupwin();
        Self { win }
    }
}

unsafe impl Sync for Terminal {}
