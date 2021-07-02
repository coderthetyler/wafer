
pub struct CircularVec<T: Copy + Clone + Sized> {
    contents: Vec<T>,
    head: usize,
    len: usize,
}

impl<T: Copy + Clone + Sized + Default> CircularVec<T> {
    /// Create a new circular vec with the fixed length `len`.
    /// The default value for the type is used to fill the vec.
    pub fn with_len(len: usize) -> Self {
        Self::with_len_and_default(len, T::default())
    }
}

impl<T: Copy + Clone + Sized> CircularVec<T> {
    /// Create a new circular vec with the fixed length `len`.
    /// The provided `default` value is used to populate the vec.
    pub fn with_len_and_default(len: usize, default: T) -> Self {
        Self {
            contents: vec![default; len],
            head: 0,
            len,
        }
    }

    /// Advance the head.
    pub fn advance(&mut self) {
        self.head = (self.head + self.len + 1) % self.len;
    }

    /// Replace the value at the head.
    pub fn replace(&mut self, new_value: T) -> T {
        let replaced_value = self.contents[self.head];
        self.contents[self.head] = new_value;
        replaced_value
    }

    /// Iterator that runs forward starting at the `head`, inclusive.
    pub fn iter(&self) -> CircularVecIter<T> {
        CircularVecIter {
            vec: self,
            direction: Direction::Forward,
            index: 0,
        }
    }

    /// Iterator that runs in reverse starting at the `head`, inclusive.
    pub fn iter_rev(&self) -> CircularVecIter<T> {
        CircularVecIter {
            vec: self,
            direction: Direction::Reverse,
            index: 0,
        }
    }
}

enum Direction {
    Forward,
    Reverse,
}

pub struct CircularVecIter<'vec, T: Copy + Clone + Sized> {
    vec: &'vec CircularVec<T>,
    direction: Direction,
    index: usize,
}

impl<'vec, T: Copy + Clone + Sized> Iterator for CircularVecIter<'vec, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index == self.vec.len {
            None
        } else {
            let len = self.vec.len;
            let iter_head = match self.direction {
                Direction::Forward => (self.vec.head + len + self.index) % len,
                Direction::Reverse => (self.vec.head + len - self.index) % len,
            };
            let item = self.vec.contents[iter_head];
            self.index += 1;
            Some(item)
        }
    }
}
