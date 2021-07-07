pub struct CircularVec<T: Clone + Sized> {
    contents: Vec<T>,
    head: usize,
    len: usize,
}

impl<T: Clone + Sized + Default> CircularVec<T> {
    /// Create a new circular vec with the fixed length `len`.
    /// The default value for the type is used to fill the vec.
    pub fn with_len(len: usize) -> Self {
        Self::with_len_and_default(len, T::default())
    }
}

impl<T: Clone + Sized> CircularVec<T> {
    /// Create a new circular vec with the fixed length `len`.
    /// The provided `default` value is used to populate the vec.
    pub fn with_len_and_default(len: usize, default: T) -> Self {
        Self {
            contents: vec![default; len],
            head: 0,
            len,
        }
    }

    /// Maximum number of elements in the vec.
    /// This is a fixed value.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Mutable reference to the head element.
    pub fn head_mut(&mut self) -> &mut T {
        &mut self.contents[self.head]
    }

    /// Immutable reference to the head element.
    pub fn head(&self) -> &T {
        &self.contents[self.head]
    }

    /// Get a mutable reference to the element with the given `index`.
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        &mut self.contents[(index % self.len + self.head + self.len) % self.len]
    }

    /// Get an immutable reference to the element with the given `index`.
    pub fn get(&self, index: usize) -> &T {
        &self.contents[(index % self.len + self.head + self.len) % self.len]
    }

    /// Advance the head forward.
    pub fn advance(&mut self) {
        self.head = (self.head + self.len + 1) % self.len;
    }

    /// Replace the value at the head.
    pub fn replace(&mut self, new_value: T) -> T {
        std::mem::replace(&mut self.contents[self.head], new_value)
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

pub struct CircularVecIter<'vec, T: Clone + Sized> {
    vec: &'vec CircularVec<T>,
    direction: Direction,
    index: usize,
}

impl<'vec, T: Clone + Sized> Iterator for CircularVecIter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<&'vec T> {
        let len = self.vec.len;
        if self.index == len {
            None
        } else {
            let iter_head = match self.direction {
                Direction::Forward => (self.vec.head + len + self.index) % len,
                Direction::Reverse => (self.vec.head + len - self.index - 1) % len,
            };
            let item = &self.vec.contents[iter_head];
            self.index += 1;
            Some(item)
        }
    }
}
