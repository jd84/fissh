/// An entry in the slot table
pub enum Entry<T> {
    Used(T),
    Empty(usize),
}

/// A slot storeage with persistent keys
/// on insert you get back an key
pub struct Slot<T> {
    entries: Vec<Entry<T>>,
    next: usize,
    len: usize,
}

/// A immutable iterator for entries in Slot
pub struct Iter<'a, T> {
    inner: std::slice::Iter<'a, Entry<T>>,
    curr: usize,
}

/// A mutable iterator for entries in Slot
pub struct IterMut<'a, T> {
    inner: std::slice::IterMut<'a, Entry<T>>,
    curr: usize,
}

// === Slot ===

impl<T> Slot<T> {
    /// Create an empty Slot
    pub fn new() -> Slot<T> {
        Slot {
            entries: Vec::new(),
            next: 0,
            len: 0,
        }
    }

    /// Create a Slot with capacity
    pub fn with_capacity(capacity: usize) -> Slot<T> {
        let mut slot = Slot {
            entries: Vec::with_capacity(capacity),
            next: 0,
            len: 0,
        };

        for i in 0..capacity {
            slot.entries.push(Entry::Empty(i + 1));
        }

        slot
    }

    pub fn insert(&mut self, val: T) -> usize {
        let key = self.next;
        self.insert_at(key, val);

        key
    }

    fn insert_at(&mut self, key: usize, val: T) {
        self.len += 1;

        if key == self.entries.len() {
            self.entries.push(Entry::Used(val));
            self.next = key + 1;
        } else {
            let prev = std::mem::replace(&mut self.entries[key], Entry::Used(val));

            match prev {
                Entry::Empty(next) => {
                    self.next = next;
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn get(&self, key: usize) -> &T {
        match self.entries[key] {
            Entry::Used(ref v) => v,
            _ => unreachable!(),
        }
    }

    pub fn get_mut(&mut self, key: usize) -> &mut T {
        match self.entries[key] {
            Entry::Used(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: self.entries.iter(),
            curr: 0,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            inner: self.entries.iter_mut(),
            curr: 0,
        }
    }
}

// === Iter ===

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            let curr = self.curr;
            self.curr += 1;

            if let Entry::Used(ref v) = *entry {
                return Some((curr, v));
            }
        }

        None
    }
}

// === IterMut ===

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            let curr = self.curr;
            self.curr += 1;

            if let Entry::Used(ref mut v) = *entry {
                return Some((curr, v));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
