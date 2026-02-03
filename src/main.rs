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
            Some(data_index) => self.data.get(*data_index)
        }
    }

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

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        let data_index = *self.index.get(idx)?;
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
}

fn main() {
    let mut container = FastContainer::<isize>::default();
    container.add(1);
    container.add(2);
    container.add(3);
    container.add(4);

    println!("{:?}", container);

    for i in 0..4 {
        println!("container.get({:?}) = {:?}", i, container.get(i));
    }

    let m = container.remove(1);

    println!("Removed element with id {}, got {:?}", 1, m);
    println!("{:?}", container);

    println!("Removed element with id {}, got {:?}", 0, container.remove(0));
    println!("{:?}", container);

    let mut id = container.add(5);
    println!("{:?}", container);
    println!("container.get({:?}) = {:?}", id, container.get(id));

    id = 2;
    println!("container.get({:?}) = {:?}", id, container.get(id));


    // let mut container = FastContainer {
    //     index: vec![ 6,  1,  4,  0,  2,  3,  5,  7 ],
    //     ids:   vec![ 3,  1,  4,  5,  2,  6,  0,  7 ],
    //     data:  vec!["d","b","e","f","c","h","i","j"],
    // };
    // let index = 4;
    // println!("{:?}", container);
    // let n = container.remove(index);
    // println!("Removded {:?} from container at index {}", n, index);
    // println!("{:?}", container);
}
