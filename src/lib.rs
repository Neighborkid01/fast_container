#[derive(Default, Clone)]
pub struct FastContainer<T> {
    index: Vec<usize>,
    ids: Vec<usize>,
    data: Vec<T>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for FastContainer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_string = f.debug_struct("FastContainer");
        for (i, el) in self.data.iter().enumerate() {
            debug_string.field(&self.ids[i].to_string(), el);
        }
        debug_string.finish()
    }
}

impl<T> FastContainer<T> {
    /// Creates a new empty FastContainer
    pub fn new() -> Self {
        Self {
            index: Vec::new(),
            ids: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Gets an optional reference to an element by its ID
    pub fn get(&self, id: usize) -> Option<&T> {
        match self.index.get(id) {
            None => None,
            Some(data_index) => self.data.get(*data_index),
        }
    }

    /// Adds an element to the container and returns its ID
    pub fn add(&mut self, el: T) -> usize {
        let index_len = self.index.len();
        let data_len = self.data.len();
        assert!(data_len <= index_len, "data.len() cannot be greater than index.len()");

        if data_len == index_len {
            self.index.push(index_len);
            self.ids.push(index_len);
        }

        self.data.push(el);
        self.ids[data_len]
    }

    /// Removes an element from the container by its ID
    pub fn remove(&mut self, id: usize) -> Option<T> {
        let data_index = *self.index.get(id)?;
        self.data.get(data_index)?;

        let last_index = self.data.len() - 1;
        if data_index < last_index {
            self.data.swap(data_index, last_index);
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
            "FastContainer {{\n  index: {:?},\n  ids: {:?},\n  data: {:?}\n}}",
            self.index, self.ids, self.data
        )
    }

    /// Returns an iterator over all id-value pairs in the container. The iterator element type is (usize, &'a T)
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            container: self,
            position: 0,
        }
    }

    /// Returns an iterator over the valid IDs in the container
    pub fn ids(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter().map(|(id, _)| id)
    }
}

/// Iterator over IDs and references to elements in a FastContainer
pub struct Iter<'a, T> {
    container: &'a FastContainer<T>,
    position: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.container.data.len() {
            return None;
        }

        let current_pos = self.position;
        self.position += 1;

        let id = self.container.ids[current_pos];
        Some((id, &self.container.data[current_pos]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_work() {
        let mut container = FastContainer::<isize>::new();
        let id1 = container.add(1);
        let id2 = container.add(2);
        let id3 = container.add(3);
        let id4 = container.add(4);

        assert_eq!(container.get(id1), Some(&1));
        assert_eq!(container.get(id2), Some(&2));
        assert_eq!(container.get(id3), Some(&3));
        assert_eq!(container.get(id4), Some(&4));
    }

    #[test]
    fn ids_are_stable_when_removing_from_start() {
        let mut container = FastContainer::<isize>::new();
        let id1 = container.add(1);
        let id2 = container.add(2);
        let id3 = container.add(3);
        let id4 = container.add(4);

        let removed = container.remove(id1);
        assert_eq!(removed, Some(1));

        assert_eq!(container.get(id1), None);
        assert_eq!(container.get(id2), Some(&2));
        assert_eq!(container.get(id3), Some(&3));
        assert_eq!(container.get(id4), Some(&4));
    }

    #[test]
    fn ids_are_stable_when_removing_from_middle() {
        let mut container = FastContainer::<isize>::new();
        let id1 = container.add(1);
        let id2 = container.add(2);
        let id3 = container.add(3);
        let id4 = container.add(4);

        let removed = container.remove(id2);
        assert_eq!(removed, Some(2));

        assert_eq!(container.get(id1), Some(&1));
        assert_eq!(container.get(id2), None);
        assert_eq!(container.get(id3), Some(&3));
        assert_eq!(container.get(id4), Some(&4));
    }

    #[test]
    fn ids_are_stable_when_removing_from_end() {
        let mut container = FastContainer::<isize>::new();
        let id1 = container.add(1);
        let id2 = container.add(2);
        let id3 = container.add(3);
        let id4 = container.add(4);

        let removed = container.remove(id4);
        assert_eq!(removed, Some(4));

        assert_eq!(container.get(id1), Some(&1));
        assert_eq!(container.get(id2), Some(&2));
        assert_eq!(container.get(id3), Some(&3));
        assert_eq!(container.get(id4), None);
    }

    #[test]
    fn ids_are_reused_after_removal() {
        let mut container = FastContainer::<isize>::new();
        let id1 = container.add(1);
        let id2 = container.add(2);
        let id3 = container.add(3);
        let id4 = container.add(4);

        container.remove(id2);
        container.remove(id4);

        assert_eq!(container.get(id1), Some(&1));
        assert_eq!(container.get(id2), None);
        assert_eq!(container.get(id3), Some(&3));
        assert_eq!(container.get(id4), None);

        let id5 = container.add(5);
        let id6 = container.add(6);

        // IDs are reused after free
        assert_eq!(id5, id4);
        assert_eq!(id6, id2);

        assert_eq!(container.get(id1), Some(&1));
        assert_eq!(container.get(id2), Some(&6));
        assert_eq!(container.get(id3), Some(&3));
        assert_eq!(container.get(id4), Some(&5));
        assert_eq!(container.get(id5), Some(&5));
        assert_eq!(container.get(id6), Some(&6));
    }

    #[test]
    fn getting_invalid_index_returns_none() {
        let container = FastContainer::<isize>::new();
        assert_eq!(container.get(0), None);
        assert_eq!(container.get(100), None);
    }

    #[test]
    fn removing_invalid_index_returns_none() {
        let mut container = FastContainer::<isize>::new();
        container.add(1);

        assert_eq!(container.remove(100), None);
    }

    #[test]
    fn iter_yields_all_elements() {
        let mut container = FastContainer::new();
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
        let mut container = FastContainer::new();
        container.add(1);
        let id = container.add(2);
        container.add(3);

        container.remove(id);

        let collected: Vec<_> = container.iter().collect();

        assert_eq!(collected.len(), 2);

        let values: Vec<_> = collected.iter().map(|(_, v)| **v).collect();
        assert!(values.contains(&1));
        assert!(!values.contains(&2)); // removed value should not be present
        assert!(values.contains(&3));
    }

    #[test]
    fn iter_works_on_empty_container() {
        let container = FastContainer::<isize>::new();
        let collected: Vec<_> = container.iter().collect();
        assert_eq!(collected.len(), 0);
    }

    #[test]
    fn iter_ids_match_get() {
        let mut container = FastContainer::new();
        container.add(1);
        container.add(2);
        container.add(3);

        for (id, value) in container.iter() {
            assert_eq!(container.get(id), Some(value));
        }
    }

    #[test]
    fn iter_count_matches_elements_after_operations() {
        let mut container = FastContainer::new();
        let id1 = container.add(1);
        container.add(2);
        let id3 = container.add(3);
        container.add(4);

        container.remove(id1);
        container.remove(id3);

        let count = container.iter().count();
        assert_eq!(count, 2);

        container.add(5);
        let count = container.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn ids_returns_all_valid_ids() {
        let mut container = FastContainer::new();
        let id1 = container.add(1);
        let id2 = container.add(2);
        let id3 = container.add(3);

        let mut ids: Vec<_> = container.ids().collect();

        assert_eq!(ids.len(), 3);

        ids.sort();
        assert_eq!(ids, [id1, id2, id3]);
    }

    #[test]
    fn ids_excludes_removed_elements() {
        let mut container = FastContainer::new();
        let id1 = container.add(1);
        let id2 = container.add(2);
        let id3 = container.add(3);

        container.remove(id2);

        let mut ids: Vec<_> = container.ids().collect();

        assert_eq!(ids.len(), 2);

        ids.sort();
        assert_eq!(ids, [id1, id3]);
    }

    #[test]
    fn ids_is_lazy() {
        let mut container = FastContainer::new();
        container.add(1);
        container.add(2);
        container.add(3);

        // Getting just the first ID should not iterate through all
        let first_id = container.ids().next();
        assert!(first_id.is_some());

        // Can get just the first 2 IDs
        let first_two: Vec<_> = container.ids().take(2).collect();
        assert_eq!(first_two.len(), 2);
    }

    #[test]
    fn ids_works_on_empty_container() {
        let container = FastContainer::<isize>::new();
        let ids: Vec<_> = container.ids().collect();
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn ids_all_are_valid() {
        let mut container = FastContainer::new();
        container.add(1);
        container.add(2);
        container.add(3);

        // Every ID from ids() should work with get()
        for id in container.ids() {
            assert!(container.get(id).is_some());
        }
    }
}
