use ascii::{AsciiChar, AsciiString};

#[derive(PartialEq, Debug, Clone, Copy)]
struct CursorPosition(usize);

pub struct Console {
    cursor: CursorPosition,
    text: AsciiString,
}

impl Console {
    pub fn new() -> Self {
        Self {
            cursor: CursorPosition(0),
            text: AsciiString::new(),
        }
    }

    /// Insert some subtext at the specified index in the console text.
    pub fn insert(&mut self, char: AsciiChar) {
        self.text.insert(self.cursor.0, char);
        self.cursor.0 += 1;
    }

    /// Backspace one character at the cursor position, if possible.
    pub fn backspace(&mut self) {
        if self.cursor.0 != 0 {
            self.text.remove(self.cursor.0 - 1);
            self.cursor.0 -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor.0 = 0;
    }

    /// Shift the cursor one character to the left, if possible.
    pub fn shift_left(&mut self) {
        if self.cursor.0 != 0 {
            self.cursor.0 -= 1;
        }
    }

    /// Shift the cursor one character to the right, if possible.
    pub fn shift_right(&mut self) {
        if self.cursor.0 != self.text.len() {
            self.cursor.0 += 1;
        }
    }

    /// Shift the cursor to the beginning of the text.
    pub fn shift_home(&mut self) {
        self.cursor.0 = 0;
    }

    /// Shift the cursor to the end of the text.
    pub fn shift_end(&mut self) {
        self.cursor.0 = self.text.len();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_in_empty_console() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        assert_eq!(console.text.len(), 1);
        assert_eq!(AsciiString::from_ascii("A").unwrap(), console.text);
    }

    #[test]
    fn insert_multiple_chars() {
        let mut console = Console::new();
        console.insert(AsciiChar::H);
        console.insert(AsciiChar::e);
        console.insert(AsciiChar::l);
        console.insert(AsciiChar::l);
        console.insert(AsciiChar::o);
        assert_eq!(AsciiString::from_ascii("Hello").unwrap(), console.text);
    }

    #[test]
    fn insert_after_shift_home() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.shift_home();
        console.insert(AsciiChar::C);
        assert_eq!(AsciiString::from_ascii("CAB").unwrap(), console.text);
    }

    #[test]
    fn insert_after_shift_end() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        console.shift_home();
        console.insert(AsciiChar::B);
        console.shift_end();
        console.insert(AsciiChar::C);
        assert_eq!(AsciiString::from_ascii("BAC").unwrap(), console.text);
    }

    #[test]
    fn backspace_on_empty_is_noop() {
        let mut console = Console::new();
        console.backspace();
        assert!(console.text.is_empty());
    }

    #[test]
    fn backspace_removes_single_char() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.backspace();
        assert_eq!(AsciiString::from_ascii("A").unwrap(), console.text);
    }

    #[test]
    fn backspace_can_remove_all_chars() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.backspace();
        console.backspace();
        assert!(console.text.is_empty());
    }

    #[test]
    fn backspace_after_shift_left() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.insert(AsciiChar::C);
        console.shift_left();
        console.backspace();
        assert_eq!(AsciiString::from_ascii("AC").unwrap(), console.text);
    }

    #[test]
    fn shift_left_moves_cursor_left() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        console.shift_left();
        assert_eq!(CursorPosition(0), console.cursor);
    }

    #[test]
    fn shift_left_moves_cursor_right() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        console.shift_home();
        console.shift_right();
        assert_eq!(CursorPosition(1), console.cursor);
    }

    #[test]
    fn shift_left_at_home_is_noop() {
        let mut console = Console::new();
        console.shift_left();
        assert_eq!(CursorPosition(0), console.cursor);
    }

    #[test]
    fn shift_right_at_end_is_noop() {
        let mut console = Console::new();
        console.shift_right();
        assert_eq!(CursorPosition(0), console.cursor);
    }

    #[test]
    fn clear_empties_text_and_moves_cursor_home() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.insert(AsciiChar::C);
        console.clear();
        assert!(console.text.is_empty());
        assert_eq!(console.cursor, CursorPosition(0));
    }

    #[test]
    fn insert_after_clear() {
        let mut console = Console::new();
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.insert(AsciiChar::C);
        console.clear();
        console.insert(AsciiChar::X);
        console.insert(AsciiChar::Y);
        assert_eq!(AsciiString::from_ascii("XY").unwrap(), console.text);
    }
}
