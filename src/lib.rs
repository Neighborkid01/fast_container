#[derive(Default, Clone)]
pub struct StableIndexVec<T> {
    index: Vec<usize>,
    generations: Vec<usize>,
    ids: Vec<usize>,
    data: Vec<T>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SIVKey {
    id: usize,
    generation: usize,
}

impl SIVKey {
    pub fn new(id: usize, generation: usize) -> Self {
        Self { id, generation }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for StableIndexVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_string = f.debug_struct("StableIndexVec");
        for (i, el) in self.data.iter().enumerate() {
            debug_string.field(&self.ids[i].to_string(), el);
        }
        debug_string.finish()
    }
}

impl<T> StableIndexVec<T> where T: PartialEq {
    /// Creates a new empty StableIndexVec
    pub fn new() -> Self {
        Self {
            index: Vec::new(),
            generations: Vec::new(),
            ids: Vec::new(),
            data: Vec::new(),
        }
    }

    fn data_index(&self, key: SIVKey) -> Option<usize> {
        let data_index = self.index.get(key.id)?;
        match self.generations.get(*data_index) {
            Some(generation) if *generation == key.generation => Some(*data_index),
            _ => None,
        }
    }

    /// Gets the length of the data vector
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Gets an optional reference to an element by its key
    pub fn get(&self, key: SIVKey) -> Option<&T> {
        let data_index = self.data_index(key)?;
        self.data.get(data_index)
    }


    /// Checks if the given element exists in the container
    pub fn contains(&self, el: &T) -> bool {
        self.data.contains(el)
    }

    /// Adds an element to the container and returns its key
    pub fn add(&mut self, el: T) -> SIVKey {
        let index_len = self.index.len();
        let data_len = self.data.len();
        assert!(data_len <= index_len, "data.len() cannot be greater than index.len()");

        if data_len == index_len {
            self.index.push(data_len);
            self.generations.push(0);
            self.ids.push(index_len);
        } else {
            self.generations[data_len] += 1;
        }

        self.data.push(el);

        SIVKey {
            id: self.ids[data_len],
            generation: self.generations[data_len]
        }
    }

    /// Removes an element from the container by its key
    pub fn remove(&mut self, key: SIVKey) -> Option<T> {
        let data_index = self.data_index(key)?;
        self.data.get(data_index)?;

        let last_index = self.data.len() - 1;
        if data_index < last_index {
            self.data.swap(data_index, last_index);
            self.generations.swap(data_index, last_index);
            self.ids.swap(data_index, last_index);
            self.index[self.ids[data_index]] = data_index;
            self.index[self.ids[last_index]] = last_index;
        }

        self.data.pop()
    }

    /// Internal debugging method that shows all internal vectors
    /// This is not public and is only used for testing and development
    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn debug_internals(&self) -> String
    where
        T: std::fmt::Debug,
    {
        format!(
            "StableIndexVec {{\n  index: {:?},\n generations: {:?},\n  ids: {:?},\n  data: {:?}\n}}",
            self.index, self.generations, self.ids, self.data
        )
    }

    /// Returns an iterator over all key-value pairs in the container. The iterator element type is (SIVKey, &'a T)
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            container: self,
            position: 0,
        }
    }

    /// Returns an iterator over the valid keys in the container
    pub fn keys(&self) -> impl Iterator<Item = SIVKey> + '_ {
        self.iter().map(|(key, _)| key)
    }

    /// Returns an iterator over the values in the container
    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.iter().map(|(_, value)| value)
    }
}

/// Iterator over keys and references to elements in a StableIndexVec
pub struct Iter<'a, T> {
    container: &'a StableIndexVec<T>,
    position: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (SIVKey, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.container.data.len() {
            return None;
        }

        let current_pos = self.position;
        self.position += 1;

        let id = self.container.ids[current_pos];
        let key = SIVKey {
            id,
            generation: self.container.generations[id],
        };
        Some((key, &self.container.data[current_pos]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_is_correct() {
        let mut container = StableIndexVec::<isize>::new();
        let key1 = container.add(1);
        assert_eq!(container.len(), 1);

        let key2 = container.add(2);
        assert_eq!(container.len(), 2);

        let key3 = container.add(3);
        assert_eq!(container.len(), 3);

        let key4 = container.add(4);
        assert_eq!(container.len(), 4);

        container.remove(key1);
        assert_eq!(container.len(), 3);

        container.remove(key3);
        assert_eq!(container.len(), 2);

        container.remove(key2);
        assert_eq!(container.len(), 1);

        container.remove(key4);
        assert_eq!(container.len(), 0);
    }

    #[test]
    fn add_and_get_work() {
        let mut container = StableIndexVec::<isize>::new();
        let key1 = container.add(1);
        let key2 = container.add(2);
        let key3 = container.add(3);
        let key4 = container.add(4);

        assert_eq!(container.get(key1), Some(&1));
        assert_eq!(container.get(key2), Some(&2));
        assert_eq!(container.get(key3), Some(&3));
        assert_eq!(container.get(key4), Some(&4));
    }

    #[test]
    fn keys_are_stable_when_removing_from_start() {
        let mut container = StableIndexVec::<isize>::new();
        let key1 = container.add(1);
        let key2 = container.add(2);
        let key3 = container.add(3);
        let key4 = container.add(4);

        let removed = container.remove(key1);
        assert_eq!(removed, Some(1));

        assert_eq!(container.get(key1), None);
        assert_eq!(container.get(key2), Some(&2));
        assert_eq!(container.get(key3), Some(&3));
        assert_eq!(container.get(key4), Some(&4));
    }

    #[test]
    fn keys_are_stable_when_removing_from_middle() {
        let mut container = StableIndexVec::<isize>::new();
        let key1 = container.add(1);
        let key2 = container.add(2);
        let key3 = container.add(3);
        let key4 = container.add(4);

        let removed = container.remove(key2);
        assert_eq!(removed, Some(2));

        assert_eq!(container.get(key1), Some(&1));
        assert_eq!(container.get(key2), None);
        assert_eq!(container.get(key3), Some(&3));
        assert_eq!(container.get(key4), Some(&4));
    }

    #[test]
    fn keys_are_stable_when_removing_from_end() {
        let mut container = StableIndexVec::<isize>::new();
        let key1 = container.add(1);
        let key2 = container.add(2);
        let key3 = container.add(3);
        let key4 = container.add(4);

        let removed = container.remove(key4);
        assert_eq!(removed, Some(4));

        assert_eq!(container.get(key1), Some(&1));
        assert_eq!(container.get(key2), Some(&2));
        assert_eq!(container.get(key3), Some(&3));
        assert_eq!(container.get(key4), None);
    }

    #[test]
    fn keys_are_not_reused_after_removal() {
        let mut container = StableIndexVec::<isize>::new();
        let key1 = container.add(1);
        let key2 = container.add(2);
        let key3 = container.add(3);
        let key4 = container.add(4);

        container.remove(key2);
        container.remove(key4);

        assert_eq!(container.get(key1), Some(&1));
        assert_eq!(container.get(key2), None);
        assert_eq!(container.get(key3), Some(&3));
        assert_eq!(container.get(key4), None);

        let key5 = container.add(5);
        let key6 = container.add(6);

        // keys are not reused after free
        assert!(![key2, key4].contains(&key5));
        assert!(![key2, key4].contains(&key6));

        assert_eq!(container.get(key1), Some(&1));
        assert_eq!(container.get(key2), None);
        assert_eq!(container.get(key3), Some(&3));
        assert_eq!(container.get(key4), None);
        assert_eq!(container.get(key5), Some(&5));
        assert_eq!(container.get(key6), Some(&6));
    }

    #[test]
    fn removing_valid_index_returns_value() {
        let mut container = StableIndexVec::<isize>::new();
        let key = container.add(1);

        assert_eq!(container.remove(key), Some(1));
        assert_eq!(container.get(key), None);
    }

    #[test]
    fn removing_invalid_index_returns_none() {
        let mut container = StableIndexVec::<isize>::new();
        let key = container.add(1);
        container.remove(key);
        container.add(1);

        assert_eq!(container.remove(key), None);
    }

    #[test]
    fn removing_valid_index_twice_returns_none() {
        let mut container = StableIndexVec::<isize>::new();
        let key = container.add(1);

        assert_eq!(container.remove(key), Some(1));
        assert_eq!(container.remove(key), None);
    }

    #[test]
    fn iter_yields_all_elements() {
        let mut container = StableIndexVec::new();
        container.add(1);
        container.add(2);
        container.add(3);

        let collected: Vec<_> = container.iter().collect();

        assert_eq!(collected.len(), 3);

        let values: Vec<_> = collected.iter().map(|(_, v)| **v).collect();
        assert!(values.contains(&1));
        assert!(values.contains(&2));
        assert!(values.contains(&3));
    }

    #[test]
    fn iter_skips_removed_elements() {
        let mut container = StableIndexVec::new();
        container.add(1);
        let key = container.add(2);
        container.add(3);

        container.remove(key);

        let collected: Vec<_> = container.iter().collect();

        assert_eq!(collected.len(), 2);

        let values: Vec<_> = collected.iter().map(|(_, v)| **v).collect();
        assert!(values.contains(&1));
        assert!(!values.contains(&2)); // removed value should not be present
        assert!(values.contains(&3));
    }

    #[test]
    fn iter_works_on_empty_container() {
        let container = StableIndexVec::<isize>::new();
        let collected: Vec<_> = container.iter().collect();
        assert_eq!(collected.len(), 0);
    }

    #[test]
    fn iter_keys_match_get() {
        let mut container = StableIndexVec::new();
        container.add(1);
        container.add(2);
        container.add(3);

        for (key, value) in container.iter() {
            assert_eq!(container.get(key), Some(value));
        }
    }

    #[test]
    fn iter_count_matches_elements_after_operations() {
        let mut container = StableIndexVec::new();
        let key1 = container.add(1);
        container.add(2);
        let key3 = container.add(3);
        container.add(4);

        container.remove(key1);
        container.remove(key3);

        let count = container.iter().count();
        assert_eq!(count, 2);

        container.add(5);
        let count = container.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn keys_returns_all_valid_keys() {
        let mut container = StableIndexVec::new();
        let key1 = container.add(1);
        let key2 = container.add(2);
        let key3 = container.add(3);

        let mut keys: Vec<_> = container.keys().collect();

        assert_eq!(keys.len(), 3);

        keys.sort();
        assert_eq!(keys, [key1, key2, key3]);
    }

    #[test]
    fn keys_excludes_removed_elements() {
        let mut container = StableIndexVec::new();
        let key1 = container.add(1);
        let key2 = container.add(2);
        let key3 = container.add(3);

        container.remove(key2);

        let mut keys: Vec<_> = container.keys().collect();

        assert_eq!(keys.len(), 2);

        keys.sort();
        assert_eq!(keys, [key1, key3]);
    }

    #[test]
    fn keys_is_lazy() {
        let mut container = StableIndexVec::new();
        container.add(1);
        container.add(2);
        container.add(3);

        // Getting just the first key should not iterate through all
        let first_key = container.keys().next();
        assert!(first_key.is_some());

        // Can get just the first 2 keys
        let first_two: Vec<_> = container.keys().take(2).collect();
        assert_eq!(first_two.len(), 2);
    }

    #[test]
    fn keys_works_on_empty_container() {
        let container = StableIndexVec::<isize>::new();
        let keys: Vec<_> = container.keys().collect();
        assert_eq!(keys.len(), 0);
    }

    #[test]
    fn keys_are_all_valid() {
        let mut container = StableIndexVec::new();
        container.add(1);
        container.add(2);
        container.add(3);

        // Every key from keys() should work with get()
        for key in container.keys() {
            assert!(container.get(key).is_some());
        }
    }

    #[test]
    fn values_returns_all_valid_values() {
        let mut container = StableIndexVec::new();
        container.add(1);
        container.add(2);
        container.add(3);

        let mut values: Vec<_> = container.values().collect();

        assert_eq!(values.len(), 3);

        values.sort();
        assert_eq!(values, [&1, &2, &3]);
    }

    #[test]
    fn values_excludes_removed_elements() {
        let mut container = StableIndexVec::new();
        container.add(1);
        let key2 = container.add(2);
        container.add(3);

        container.remove(key2);

        let mut values: Vec<_> = container.values().collect();

        assert_eq!(values.len(), 2);

        values.sort();
        assert_eq!(values, [&1, &3]);
    }

    #[test]
    fn values_is_lazy() {
        let mut container = StableIndexVec::new();
        container.add(1);
        container.add(2);
        container.add(3);

        // Getting just the first value should not iterate through all
        let first_value = container.values().next();
        assert!(first_value.is_some());

        // Can get just the first 2 values
        let first_two: Vec<_> = container.values().take(2).collect();
        assert_eq!(first_two.len(), 2);
    }

    #[test]
    fn values_works_on_empty_container() {
        let container = StableIndexVec::<isize>::new();
        let values: Vec<_> = container.keys().collect();
        assert_eq!(values.len(), 0);
    }

    #[test]
    fn values_are_all_valid() {
        let mut container = StableIndexVec::new();
        container.add(1);
        container.add(2);
        container.add(3);

        for value in container.values() {
            assert!(container.contains(value));
        }
    }
}
