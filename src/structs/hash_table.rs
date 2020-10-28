/// Hash table linear-probing implementation.
pub struct HashTable<T> {
    keys: Vec<usize>,
    data: Vec<T>,
}

impl<T> HashTable<T> where T: Default + Copy {
    pub fn new(size: usize) -> HashTable<T> {
        HashTable {
            keys: vec![0; size],
            data: vec![T::default(); size],
        }
    }

    fn get_index(&self, key: usize) -> usize {
        let mut index = key % self.data.len();
        while self.keys[index] != key && self.keys[index] != 0 {
            index = (index + 1) % self.data.len();
        }
        index
    }

    pub fn set(&mut self, key: usize, value: T) {
        let index = self.get_index(key);
        self.keys[index] = key;
        self.data[index] = value;
    }

    pub fn get(&self, key: usize) -> T {
        let index = self.get_index(key);
        self.data[index]
    }

    pub fn delete(&mut self, key: usize) {
        let mut index = self.get_index(key);
        self.keys[index] = 0;
        self.data[index] = T::default();

        index = (index + 1) % self.data.len();
        while self.keys[index] != 0 {
            let k = self.keys[index];
            let v = self.data[index];
            self.keys[index] = 0;
            self.data[index] = T::default();
            self.set(k, v);
            index = (index + 1) % self.data.len();
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        let h: HashTable<i32> = HashTable::new(1);
        assert_eq!(h.size(), 1);
    }

    #[test]
    fn set_get_one() {
        let mut h = HashTable::new(2);
        h.set(1, 5);
        assert_eq!(h.get(1), 5);
    }

    #[test]
    fn set_get_two() {
        let mut h = HashTable::new(3);
        h.set(1, 5);
        h.set(2, 3);
        assert_eq!(h.get(1), 5);
        assert_eq!(h.get(2), 3);
    }

    #[test]
    fn set_delete_get() {
        let mut h = HashTable::new(4);
        h.set(1, 1);
        h.set(2, 2);
        h.set(5, 5);
        assert_eq!(h.get(1), 1);
        assert_eq!(h.get(2), 2);
        assert_eq!(h.get(5), 5);

        h.delete(1);

        assert_eq!(h.get(1), 0);
        assert_eq!(h.get(2), 2);
        assert_eq!(h.get(5), 5);
    }

    #[test]
    fn set_delete_get_same_hash() {
        let mut h = HashTable::new(5);
        h.set(1, 1);
        h.set(6, 6);
        h.set(11, 11);
        assert_eq!(h.get(1), 1);
        assert_eq!(h.get(6), 6);
        assert_eq!(h.get(11), 11);

        h.delete(6);

        assert_eq!(h.get(1), 1);
        assert_eq!(h.get(6), 0);
        assert_eq!(h.get(11), 11);
    }
}