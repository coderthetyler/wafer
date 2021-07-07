use ascii::{AsciiChar, AsciiString};

use super::{CircularVec, CircularVecIter};

#[derive(PartialEq, Debug, Clone, Copy)]
struct CursorPosition(usize);

pub struct Console {
    cursor: CursorPosition,
    text: AsciiString,
    history: CircularVec<AsciiString>,
    /// Distance from last entry in history
    index_up_history: usize,
}

impl Console {
    pub fn new(len: usize) -> Self {
        Self {
            cursor: CursorPosition(0),
            text: AsciiString::new(),
            history: CircularVec::with_len(len),
            index_up_history: 0,
        }
    }

    /// Submission history ordered from newest to oldest.
    pub fn history_newest_first(&self) -> CircularVecIter<AsciiString> {
        self.history.iter_rev()
    }

    // Submission history ordered from oldest to newest.
    pub fn history_oldest_first(&self) -> CircularVecIter<AsciiString> {
        self.history.iter()
    }

    /// Text currently entered at the command line
    pub fn get_text(&self) -> &AsciiString {
        &self.text
    }

    /// Insert some subtext at the specified index in the console text.
    pub fn insert(&mut self, char: AsciiChar) {
        self.text.insert(self.cursor.0, char);
        self.cursor.0 += 1;
    }

    /// Backspace one character at the cursor position, if possible.
    pub fn backspace(&mut self) {
        if self.cursor.0 != 0 {
            self.cursor.0 -= 1;
            self.text.remove(self.cursor.0);
        }
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
        self.cursor = CursorPosition(0);
    }

    /// Shift the cursor to the end of the text.
    pub fn shift_end(&mut self) {
        self.cursor = CursorPosition(self.text.len());
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = CursorPosition(0);
    }

    /// Get & clear the text of the console.
    /// This method returns an owned `String`.
    pub fn submit(&mut self) -> Option<String> {
        if self.text.is_empty() {
            None
        } else {
            let text = self.text.clone().to_string();
            self.history.replace(self.text.clone());
            self.history.advance();
            self.index_up_history = 0;
            self.clear();
            Some(text)
        }
    }

    /// Go back in time up the history stack, if possible.
    /// If we're already at the top, then do nothing.
    pub fn navigate_backwards(&mut self) {
        self.index_up_history = (self.index_up_history + 1).min(self.history.len());
        self.text
            .clone_from(self.history.get(self.history.len() - self.index_up_history));
        self.cursor = CursorPosition(self.text.len());
    }

    /// Go forward in time down the history stack, if possible.
    /// If we're already at the bottom, then clear the text.
    pub fn navigate_forwards(&mut self) {
        if self.index_up_history <= 1 {
            self.index_up_history = 0;
            self.clear();
        } else {
            self.index_up_history -= 1;
            self.text
                .clone_from(self.history.get(self.history.len() - self.index_up_history));
            self.cursor = CursorPosition(self.text.len());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_in_empty_console() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        assert_eq!(console.text.len(), 1);
        assert_eq!(AsciiString::from_ascii("A").unwrap(), console.text);
    }

    #[test]
    fn insert_multiple_chars() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::H);
        console.insert(AsciiChar::e);
        console.insert(AsciiChar::l);
        console.insert(AsciiChar::l);
        console.insert(AsciiChar::o);
        assert_eq!(AsciiString::from_ascii("Hello").unwrap(), console.text);
    }

    #[test]
    fn insert_after_shift_home() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.shift_home();
        console.insert(AsciiChar::C);
        assert_eq!(AsciiString::from_ascii("CAB").unwrap(), console.text);
    }

    #[test]
    fn insert_after_shift_end() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.shift_home();
        console.insert(AsciiChar::B);
        console.shift_end();
        console.insert(AsciiChar::C);
        assert_eq!(AsciiString::from_ascii("BAC").unwrap(), console.text);
    }

    #[test]
    fn backspace_on_empty_is_noop() {
        let mut console = Console::new(16);
        console.backspace();
        assert!(console.text.is_empty());
    }

    #[test]
    fn backspace_removes_single_char() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.backspace();
        assert_eq!(AsciiString::from_ascii("A").unwrap(), console.text);
    }

    #[test]
    fn backspace_can_remove_all_chars() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.backspace();
        console.backspace();
        assert!(console.text.is_empty());
    }

    #[test]
    fn backspace_after_shift_left() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.insert(AsciiChar::C);
        console.shift_left();
        console.backspace();
        assert_eq!(AsciiString::from_ascii("AC").unwrap(), console.text);
    }

    #[test]
    fn shift_left_moves_cursor_left() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.shift_left();
        assert_eq!(CursorPosition(0), console.cursor);
    }

    #[test]
    fn shift_left_moves_cursor_right() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.shift_home();
        console.shift_right();
        assert_eq!(CursorPosition(1), console.cursor);
    }

    #[test]
    fn shift_left_at_home_is_noop() {
        let mut console = Console::new(16);
        console.shift_left();
        assert_eq!(CursorPosition(0), console.cursor);
    }

    #[test]
    fn shift_right_at_end_is_noop() {
        let mut console = Console::new(16);
        console.shift_right();
        assert_eq!(CursorPosition(0), console.cursor);
    }

    #[test]
    fn clear_empties_text_and_moves_cursor_home() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.insert(AsciiChar::C);
        console.clear();
        assert!(console.text.is_empty());
        assert_eq!(console.cursor, CursorPosition(0));
    }

    #[test]
    fn insert_after_clear() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.insert(AsciiChar::B);
        console.insert(AsciiChar::C);
        console.clear();
        console.insert(AsciiChar::X);
        console.insert(AsciiChar::Y);
        assert_eq!(AsciiString::from_ascii("XY").unwrap(), console.text);
    }

    // #[test]
    // fn submit_noop_with_empty_text() {
    //     let mut console = Console::new();
    //     assert_eq!(None, console.submit());
    // }

    #[test]
    fn submit_adds_history() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.submit();
        console.navigate_backwards();
        assert_eq!(AsciiString::from_ascii("A").unwrap(), console.text);
    }

    #[test]
    fn submit_clears_text() {
        let mut console = Console::new(16);
        console.insert(AsciiChar::A);
        console.submit();
        assert!(console.text.is_empty());
    }
}
