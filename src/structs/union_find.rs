pub struct UnionFind {
    data: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        assert!(size > 0);

        let mut data = vec![0; size];
        for (i, e) in data.iter_mut().enumerate() {
            *e = i;
        }

        UnionFind {
            data: data,
            count: size,
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        assert!(p < self.data.len());
        assert!(q < self.data.len());

        if self.data[p] != self.data[q] {
            self.data[p] = self.data[q];
            self.count -= 1;    
        }
    }

    pub fn find(&self, p: usize) -> usize {
        assert!(p < self.data.len());

        self.data[p]
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        assert!(p < self.data.len());
        assert!(q < self.data.len());

        self.data[p] == self.data[q]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_element() {
        let uf = UnionFind::new(1);
        assert_eq!(uf.count(), 1);
        assert_eq!(uf.find(0), 0);
    }

    #[test]
    fn two_elements() {
        let mut uf = UnionFind::new(2);
        assert_eq!(uf.count(), 2);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.connected(0, 1), false);

        uf.union(0, 1);
        assert_eq!(uf.count(), 1);
        assert_eq!(uf.connected(0, 1), true);
    }
}