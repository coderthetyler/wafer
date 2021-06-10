use std::cmp::Ordering;

// TODO implement iterators for generational index arrays

/// Index into a [`GenerationalIndexArray`].
#[derive(Clone, Copy)]
pub struct GenerationalIndex {
    index: usize,
    generation: u64,
}

impl GenerationalIndex {
    pub fn none() -> GenerationalIndex {
        Self {
            index: 0,
            generation: 0,
        }
    }
}

/// Entry in a [`GenerationalIndexAllocator`].
struct AllocatorEntry {
    is_live: bool,
    generation: u64,
}

/// Allocates & deallocates generational indices.
pub struct GenerationalIndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl GenerationalIndexAllocator {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            free: vec![],
        }
    }

    /// Allocate an index, potentially reusing memory from a previously deallocated index.
    pub fn allocate(&mut self) -> GenerationalIndex {
        match self.free.pop() {
            Some(index) => {
                let entry = &mut self.entries[index];
                entry.generation += 1;
                GenerationalIndex {
                    index,
                    generation: entry.generation,
                }
            }
            None => {
                let entry = AllocatorEntry {
                    is_live: true,
                    generation: 1,
                };
                let index = self.entries.len();
                self.entries.push(entry);
                GenerationalIndex {
                    index,
                    generation: 1,
                }
            }
        }
    }

    /// Attempt to deallocate an index.
    ///
    /// Returns `false` if the index was previously deallocated.
    pub fn deallocate(&mut self, index: GenerationalIndex) -> bool {
        if !self.is_live(index) {
            false
        } else {
            let entry = &mut self.entries[index.index];
            entry.is_live = false;
            self.free.push(index.index);
            true
        }
    }

    /// Returns `true` only if the entry has not yet been deallocated.
    pub fn is_live(&self, index: GenerationalIndex) -> bool {
        let entry = &self.entries[index.index];
        entry.is_live && entry.generation == index.generation
    }
}

struct ArrayEntry<T> {
    value: T,
    generation: u64,
}

pub struct GenerationalIndexVec<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexVec<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Set the value for some index. The `index` must be at least as new as the existing array entry, if any.
    pub fn set(&mut self, index: GenerationalIndex, value: T) {
        // Need to ensure the wrapped vec can be accessed at the index
        // e.g. if index=10 & len=5, then 5 None values need to be pushed to the vec
        let new_entry = Some(ArrayEntry {
            value,
            generation: index.generation,
        });
        let len = self.0.len();
        match index.index.cmp(&len) {
            Ordering::Less => self.0[index.index] = new_entry,
            Ordering::Equal => self.0.push(new_entry),
            Ordering::Greater => {
                self.0.resize_with(index.index + 1, || None);
                self.0[index.index] = new_entry;
            }
        }
    }

    /// Get an immutable reference to the value for some index. Returns `None` if there is no such value.
    pub fn get(&self, index: GenerationalIndex) -> Option<&T> {
        let entry_opt = self.0.get(index.index)?;
        if let Some(entry) = entry_opt {
            if entry.generation == index.generation {
                return Some(&entry.value);
            }
        }
        None
    }

    /// Get a mutable reference to the value for some index. Returns `None` if there is no such value.
    pub fn get_mut(&mut self, index: GenerationalIndex) -> Option<&mut T> {
        let entry_opt = self.0.get_mut(index.index)?;
        if let Some(entry) = entry_opt {
            if entry.generation == index.generation {
                return Some(&mut entry.value);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index(index: usize, generation: u64) -> GenerationalIndex {
        GenerationalIndex { index, generation }
    }

    #[test]
    fn set_consecutive_values() {
        let mut vec = GenerationalIndexVec::<u32>::new();
        let index0gen1 = index(0, 1);
        let index1gen1 = index(1, 1);
        vec.set(index0gen1, 5);
        vec.set(index1gen1, 3);
        assert_eq!(vec.0.len(), 2);
        assert_eq!(vec.get(index0gen1), Some(&5));
        assert_eq!(vec.get(index1gen1), Some(&3));
    }

    #[test]
    fn overwrite_value_with_same_generation() {
        let mut vec = GenerationalIndexVec::<u32>::new();
        let index0gen1 = index(0, 1);
        vec.set(index0gen1, 5);
        vec.set(index0gen1, 2);
        vec.set(index0gen1, 10);
        assert_eq!(vec.0.len(), 1);
        assert_eq!(vec.get(index0gen1), Some(&10));
    }

    #[test]
    fn overwrite_value_with_new_generation() {
        let mut vec = GenerationalIndexVec::<u32>::new();
        let index0gen1 = index(0, 1);
        let index0gen2 = index(0, 2);
        vec.set(index0gen1, 10);
        vec.set(index0gen2, 4);
        assert_eq!(vec.0.len(), 1);
        assert_eq!(vec.get(index0gen1), None);
        assert_eq!(vec.get(index0gen2), Some(&4));
    }

    #[test]
    fn set_with_index_gap() {
        let mut vec = GenerationalIndexVec::<u32>::new();
        let index0gen1 = index(0, 1);
        let index10gen1 = index(10, 1);
        vec.set(index0gen1, 5);
        vec.set(index10gen1, 3);
        assert_eq!(vec.0.len(), 11);
        assert_eq!(vec.get(index0gen1), Some(&5));
        assert_eq!(vec.get(index10gen1), Some(&3));
    }

    #[test]
    fn overwrite_value_in_place() {
        let mut vec = GenerationalIndexVec::<u32>::new();
        let index0gen1 = index(0, 1);
        vec.set(index0gen1, 4);
        let entry = vec.get_mut(index0gen1).unwrap();
        *entry += 2;
        assert_eq!(vec.0.len(), 1);
        assert_eq!(vec.get(index0gen1), Some(&6));
    }

    #[test]
    fn allocate_index() {
        let mut allocator = GenerationalIndexAllocator::new();
        let index0gen1 = allocator.allocate();
        assert_eq!(index0gen1.index, 0);
        assert_eq!(index0gen1.generation, 1);
    }

    #[test]
    fn is_live() {
        let mut allocator = GenerationalIndexAllocator::new();
        let index0gen1 = allocator.allocate();
        assert_eq!(allocator.is_live(index0gen1), true);
        assert_eq!(allocator.deallocate(index0gen1), true);
        assert_eq!(allocator.is_live(index0gen1), false);
    }

    #[test]
    fn deallocate_twice_returns_false() {
        let mut allocator = GenerationalIndexAllocator::new();
        let index0gen1 = allocator.allocate();
        assert_eq!(allocator.deallocate(index0gen1), true);
        assert_eq!(allocator.deallocate(index0gen1), false);
    }

    #[test]
    fn reallocate_reuses_old_index() {
        let mut allocator = GenerationalIndexAllocator::new();
        let index0gen1 = allocator.allocate();
        allocator.deallocate(index0gen1);
        let index0gen2 = allocator.allocate();
        assert_eq!(index0gen2.index, 0);
        assert_eq!(index0gen2.generation, 2);
    }

    #[test]
    fn just_do_alot_of_allocations() {
        let mut allocator = GenerationalIndexAllocator::new();
        for i in 0..10 {
            let index = allocator.allocate();
            assert_eq!(index.index, i);
            assert_eq!(index.generation, 1);
        }
    }

    #[test]
    fn get_none_gives_none() {
        let mut vec = GenerationalIndexVec::<u32>::new();
        let none = GenerationalIndex::none();
        let index0gen1 = index(0, 1);
        vec.set(index0gen1, 5);
        assert_eq!(vec.get(none), None);
    }
}
