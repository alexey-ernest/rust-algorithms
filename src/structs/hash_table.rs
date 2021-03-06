use std::mem;

/// Auto-resizable hash table linear-probing implementation with O(1) amortized cost.
pub struct HashTable<T> {
    keys: Vec<Option<usize>>,
    data: Vec<Option<T>>,
    count: usize,
    auto_resize: bool,
}

const INIT_SIZE: usize = 997;
const MAX_FILL_PERCENT: f32 = 1.0/4.0;
const MIN_FILL_PERCENT: f32 = 1.0/8.0;

impl<T> HashTable<T> {
    pub fn new() -> Self {
        HashTable::new_size(INIT_SIZE)
    }

    pub fn new_size(size: usize) -> Self {
        let mut keys = Vec::with_capacity(size);
        keys.resize_with(size, || None);
        
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, || None);

        HashTable {
            keys,
            data,
            count: 0,
            auto_resize: true,
        }
    }

    pub fn set_autoresize(&mut self, val: bool) {
        self.auto_resize = val;
    }

    fn get_index(&self, key: usize) -> usize {
        let mut index = key % self.data.len();
        while let Some(ref k) = &self.keys[index] {
            if *k == key {
                break;
            }
            index = (index + 1) % self.data.len();
        }
        index
    }

    fn rehash(&mut self, newsize: usize) {
        let mut h = HashTable::new_size(newsize);
        let keys = mem::replace(&mut self.keys, Vec::new());
        let data = mem::replace(&mut self.data, Vec::new());

        for (k, v) in keys.into_iter().zip(data.into_iter()) {
            match k {
                None => continue,
                Some(key) => {
                    h.set(key, v.unwrap());
                }
            }
        }
        *self = h;
    }

    pub fn set(&mut self, key: usize, value: T) {
        let index = self.get_index(key);
        self.keys[index] = Some(key);
        self.data[index] = Some(value);
        self.count += 1;

        let fill_percent = (self.count as f32)/(self.data.len() as f32);
        if self.auto_resize && fill_percent >= MAX_FILL_PERCENT {
            self.rehash(self.data.len()*2);
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        let index = self.get_index(key);
        self.data[index].as_ref()
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        let index = self.get_index(key);
        self.data[index].as_mut()
    }

    pub fn delete(&mut self, key: usize) {
        let mut index = self.get_index(key);
        if !self.keys[index].is_some() {
            return;
        }

        self.keys[index].take();
        self.data[index].take();
        self.count -= 1;

        index = (index + 1) % self.data.len();
        while self.keys[index].is_some() {
            let k = self.keys[index].take();
            let v = self.data[index].take();
            self.count -= 1;
            self.set(k.unwrap(), v.unwrap());
            index = (index + 1) % self.data.len();
        }

        let fill_percent = (self.count as f32)/(self.data.len() as f32);
        if self.auto_resize && fill_percent <= MIN_FILL_PERCENT {
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
mod tests {
    extern crate test;

    use super::*;
    use rand::Rng;
    use test::Bencher;

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
        assert_eq!(h.get(1), Some(&5));
    }

    #[test]
    fn set_get_mut() {
        let mut h = HashTable::new();
        h.set(1, 5);
        assert_eq!(h.get_mut(1), Some(&mut 5));

        h.get_mut(1).map(|val| {
            *val = 6;
        });
        assert_eq!(h.get(1), Some(&6));
    }

    #[test]
    fn set_get_two() {
        let mut h = HashTable::new();
        h.set(1, 5);
        h.set(2, 3);
        assert_eq!(h.get(1), Some(&5));
        assert_eq!(h.get(2), Some(&3));
    }

    #[test]
    fn set_delete_get() {
        let mut h = HashTable::new();
        h.set(1, 1);
        h.set(2, 2);
        h.set(5, 5);
        assert_eq!(h.get(1), Some(&1));
        assert_eq!(h.get(2), Some(&2));
        assert_eq!(h.get(5), Some(&5));

        h.delete(1);

        assert_eq!(h.get(1), None);
        assert_eq!(h.get(2), Some(&2));
        assert_eq!(h.get(5), Some(&5));
    }

    #[test]
    fn set_delete_get_same_hash() {
        let mut h = HashTable::new();
        h.set(1, 1);
        h.set(6, 6);
        h.set(11, 11);
        assert_eq!(h.get(1), Some(&1));
        assert_eq!(h.get(6), Some(&6));
        assert_eq!(h.get(11), Some(&11));

        h.delete(6);

        assert_eq!(h.get(1), Some(&1));
        assert_eq!(h.get(6), None);
        assert_eq!(h.get(11), Some(&11));
    }

    #[test]
    fn double_and_rehash() {
        let mut h = HashTable::new();
        for i in 0..2000 {
            h.set(i, i);
        }

        assert_eq!(h.len(), 2000);
        assert_eq!(h.cap(), 15952);

        for i in 0..2000 {
            assert_eq!(h.get(i), Some(&i));
        }

        for i in 0..1000 {
            h.delete(i);
        }

        assert_eq!(h.len(), 1000);
        assert_eq!(h.cap(), 7976);

        for i in 1000..2000 {
            assert_eq!(h.get(i), Some(&i));
        }
    }

    #[bench]
    fn bench_set_1(b: &mut Bencher) {
        let mut h = HashTable::new();

        b.iter(move|| {
            h.set(1, 1);
        });
    }

    #[bench]
    fn bench_set_1000_cons(b: &mut Bencher) {
        let mut h = HashTable::new_size(10000);
        h.set_autoresize(false);

        b.iter(move|| {
            for i in 0..1000 {
                h.set(i, i);    
            }
        });
    }

    #[bench]
    fn bench_delete_1000_cons(b: &mut Bencher) {
        let mut h = HashTable::new_size(10000);
        h.set_autoresize(false);
        for i in 0..1000 {
            h.set(i, i);
        }

        b.iter(move|| {
            for i in 0..1000 {
                h.delete(i);
            }
        });
    }

    #[bench]
    fn bench_delete_1000_inv_cons(b: &mut Bencher) {
        let mut h = HashTable::new_size(10000);
        h.set_autoresize(false);
        for i in 0..1000 {
            h.set(i, i);
        }

        b.iter(move|| {
            for i in (0..1000).rev() {
                h.delete(i);
            }
        });
    }

    #[bench]
    fn bench_set_1000_rand(b: &mut Bencher) {
        let mut h = HashTable::new_size(10000);
        h.set_autoresize(false);

        let mut rng = rand::thread_rng();
        let mut vals: Vec<usize> = vec![];
        for _ in 0..1000 {
            vals.push(rng.gen_range(0, 1000) as usize);
        }

        b.iter(move|| {
            for &i in vals.iter() {
                h.set(i, i);
            }
        });
    }

    #[bench]
    fn bench_set_delete_1000_rand(b: &mut Bencher) {
        let mut h = HashTable::new_size(10000);
        h.set_autoresize(false);

        let mut rng = rand::thread_rng();
        let mut vals: Vec<usize> = vec![];
        for _ in 0..1000 {
            let val = rng.gen_range(0, 1000) as usize;
            vals.push(val);
            h.set(val, val);
        }

        b.iter(move|| {
            for &i in vals.iter() {
                h.delete(i);
            }
        });
    }

    #[bench]
    fn bench_set_1000_collision_fill_05(b: &mut Bencher) {
        let mut h = HashTable::new_size(2000);

        let mut rng = rand::thread_rng();
        let mut vals: Vec<usize> = vec![];
        for _ in 0..1000 {
            vals.push(rng.gen_range(0, 10000) as usize);
        }

        b.iter(move|| {
            for &i in vals.iter() {
                h.set(i, i);
            }
        });
    }
}