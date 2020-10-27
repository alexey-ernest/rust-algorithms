/// Min-oriented heap structure
pub struct Heap<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Heap<T> where T: std::cmp::PartialOrd + Copy + Default {
    pub fn new(size: usize) -> Heap<T> {
        assert!(size > 0);

        Heap {
            size: 0,
            data: vec![T::default(); size+1],
        }
    }

    pub fn from(list: & [T]) -> Heap<T> {
        let mut h = Heap::new(list.len());
        for e in list.iter() {
            h.push(*e);
        }
        h
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn top(&self) -> T {
        assert!(self.size > 0, "the heap is empty");

        self.data[1]
    }

    pub fn push(&mut self, el: T) {
        assert!(self.size + 1 < self.data.len(), "the heap is full");
        
        self.data[self.size+1] = el;
        self.swim(self.size+1);
        self.size += 1;
    }

    pub fn pop(&mut self) -> T {
        assert!(self.size > 0, "the heap is empty");

        let x = self.data[1];
        self.data.swap(1, self.size);
        self.size -= 1;
        self.sink(1);
        x
    }

    fn swim(&mut self, i: usize) {
        let mut k = i;
        while k > 1 {
            let p = k/2;
            if self.data[k] < self.data[p] {
                self.data.swap(k, p);
            }
            k = p;
        }
    }

    fn sink(&mut self, i: usize) {
        let mut k = i;
        while 2*k < self.size+1 {
            let mut p = k*2;
            if p+1 < self.size+1 && self.data[p+1] < self.data[p] {
                // choosing minimum of two children
                p += 1;
            }
            if self.data[p] < self.data[k] {
                self.data.swap(p, k);
            }
            k = p;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_element() {
        let mut h = Heap::new(1);
        assert_eq!(h.size(), 0);

        h.push(1);
        assert_eq!(h.top(), 1);
        assert_eq!(h.size(), 1);
    }

    #[test]
    fn two_elements() {
        let mut h = Heap::new(2);
        h.push(2);
        h.push(1);

        assert_eq!(h.top(), 1);
        assert_eq!(h.size(), 2);
        h.pop();
        assert_eq!(h.top(), 2);
        assert_eq!(h.size(), 1);
    }

    #[test]
    fn three_elements() {
        let mut h = Heap::new(3);
        h.push(2);
        h.push(1);
        h.push(3);

        assert_eq!(h.top(), 1);
        assert_eq!(h.size(), 3);
        h.pop();
        assert_eq!(h.top(), 2);
        assert_eq!(h.size(), 2);
        h.pop();
        assert_eq!(h.top(), 3);
        assert_eq!(h.size(), 1);
    }

    #[test]
    fn from_list() {
        let h = Heap::from(&vec![2, 3, 1]);

        assert_eq!(h.top(), 1);
        assert_eq!(h.size(), 3);
    }
}
