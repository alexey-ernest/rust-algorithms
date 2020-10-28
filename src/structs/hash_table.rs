/// Hash table linear-probing implementation.
pub struct HashTable<T> {
    keys: Vec<usize>,
    data: Vec<T>,
    count: usize,
}

const INIT_SIZE: usize = 997;
const MAX_FILL_PERCENT: f32 = 1.0/4.0;
const MIN_FILL_PERCENT: f32 = 1.0/8.0;

impl<T> HashTable<T> where T: Default + Copy {
    pub fn new() -> HashTable<T> {
        HashTable {
            keys: vec![0; INIT_SIZE],
            data: vec![T::default(); INIT_SIZE],
            count: 0,
        }
    }

    fn get_index(&self, key: usize) -> usize {
        let mut index = key % self.data.len();
        while self.keys[index] != key && self.keys[index] != 0 {
            index = (index + 1) % self.data.len();
        }
        index
    }

    fn rehash(&mut self, newsize: usize) {
        let mut h = HashTable {
            keys: vec![0; newsize],
            data: vec![T::default(); newsize],
            count: 0,
        };
        for (i, &k) in self.keys.iter().enumerate() {
            if k == 0 {
                continue;
            }
            h.set(k, self.data[i]);
        }
        *self = h;
    }

    pub fn set(&mut self, key: usize, value: T) {
        let index = self.get_index(key);
        self.keys[index] = key;
        self.data[index] = value;
        self.count += 1;

        let fill_percent = (self.count as f32)/(self.data.len() as f32);
        if fill_percent >= MAX_FILL_PERCENT {
            self.rehash(2*self.data.len());
        }
    }

    pub fn get(&self, key: usize) -> T {
        let index = self.get_index(key);
        self.data[index]
    }

    pub fn delete(&mut self, key: usize) {
        let mut index = self.get_index(key);
        if self.keys[index] == 0 {
            return;
        }

        self.keys[index] = 0;
        self.data[index] = T::default();
        self.count -= 1;

        index = (index + 1) % self.data.len();
        while self.keys[index] != 0 {
            let k = self.keys[index];
            let v = self.data[index];
            self.keys[index] = 0;
            self.data[index] = T::default();
            self.count -= 1;
            self.set(k, v);

            index = (index + 1) % self.data.len();
        }

        let fill_percent = (self.count as f32)/(self.data.len() as f32);
        if fill_percent <= MIN_FILL_PERCENT {
            self.rehash(self.data.len()/2);
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn cap(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        let h: HashTable<i32> = HashTable::new();
        assert_eq!(h.len(), 0);
        assert_ne!(h.cap(), 0);
    }

    #[test]
    fn set_get_one() {
        let mut h = HashTable::new();
        h.set(1, 5);
        assert_eq!(h.get(1), 5);
    }

    #[test]
    fn set_get_two() {
        let mut h = HashTable::new();
        h.set(1, 5);
        h.set(2, 3);
        assert_eq!(h.get(1), 5);
        assert_eq!(h.get(2), 3);
    }

    #[test]
    fn set_delete_get() {
        let mut h = HashTable::new();
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
        let mut h = HashTable::new();
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

    #[test]
    fn double_and_rehash() {
        let mut h = HashTable::new();
        for i in 1..2001 {
            h.set(i, i);
        }

        assert_eq!(h.len(), 2000);
        assert_eq!(h.cap(), 15952);

        for i in 1..2001 {
            assert_eq!(h.get(i), i);
        }

        for i in 1..1001 {
            h.delete(i);
        }

        assert_eq!(h.len(), 1000);
        assert_eq!(h.cap(), 7976);

        for i in 1001..2001 {
            assert_eq!(h.get(i), i);
        }
    }
}