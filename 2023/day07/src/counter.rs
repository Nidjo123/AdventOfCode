use std::collections::HashMap;
use std::hash::Hash;

pub struct Counter<T: Hash> {
    counts: HashMap<T, usize>,
}

impl<T: Eq + Hash + Copy> Counter<T> {
    pub fn new() -> Self {
        Counter {
            counts: HashMap::new(),
        }
    }

    pub fn from_iter(values: impl Iterator<Item = T>) -> Self {
        let mut counter = Self::new();
        counter.count(values);
        counter
    }

    fn count_one(&mut self, value: T) {
        self.counts
            .entry(value)
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }

    pub fn count(&mut self, values: impl Iterator<Item = T>) {
        values.for_each(|x| self.count_one(x));
    }
}

impl<T: Hash + Copy + Clone> IntoIterator for Counter<T> {
    type Item = (T, usize);
    type IntoIter = CounterIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        CounterIterator {
            current_index: 0,
            items: self.counts.into_iter().collect(),
        }
    }
}

pub struct CounterIterator<T: Copy + Clone> {
    current_index: usize,
    items: Vec<(T, usize)>,
}

impl<T: Copy + Clone> Iterator for CounterIterator<T> {
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.get(self.current_index);
        self.current_index += 1;
        item.copied()
    }
}
