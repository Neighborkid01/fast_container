#[derive(Debug, Default)]
struct FastContainer<T> {
    index: Vec<usize>,
    ids: Vec<usize>,
    data: Vec<T>,
}

impl<T> FastContainer<T> {
    pub fn get(&self, idx: usize) -> Option<&T> {
        match self.index.get(idx) {
            None => None,
            Some(data_index) => {
                let data_ref = self.data.get(*data_index);
                assert!(data_ref.is_some(), "data_index: {} somehow points to a value outside data (len = {})", *data_index, self.data.len());
                data_ref
            }
        }
    }

    pub fn push(&mut self, el: T) {
        let index_len = self.index.len();
        assert!(self.data.len() <= index_len, "data.len() cannot be greater than index.len()");

        if self.data.len() < index_len {
            self.data.push(el);
            return;
        }

        self.index.push(index_len);
        self.ids.push(index_len);
        self.data.push(el);
    }
}

fn main() {
    let mut container = FastContainer::<isize>::default();
    container.push(1);
    container.push(2);
    container.push(3);

    println!("{:?}", container);

    for i in 0..4 {
        println!("container[{:?}] = {:?}", i, container.get(i));
    }
}
