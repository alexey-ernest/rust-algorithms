/// Weighted quick-union implementation
pub struct UnionFind {
    data: Vec<usize>,
    size: Vec<usize>,
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
            size: vec![1; size],
            count: size,
        }
    }

    pub fn find(&self, p: usize) -> usize {
        assert!(p < self.data.len());

        let mut p = p;
        while p != self.data[p] {
            p = self.data[p];
        }
        p
    }

    pub fn union(&mut self, p: usize, q: usize) {
        assert!(p < self.data.len());
        assert!(q < self.data.len());

        let i = self.find(p);
        let j = self.find(q);
        if i == j {
            return;
        }

        if self.size[i] < self.size[j] {
            self.data[i] = j;
            self.size[j] += self.size[i];
        } else {
            self.data[j] = i;
            self.size[i] += self.size[j];
        }
        
        self.count -= 1;    
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        assert!(p < self.data.len());
        assert!(q < self.data.len());

        self.find(p) == self.find(q)
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

    #[test]
    fn two_components_union() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(2, 3);

        assert_eq!(uf.connected(0, 1), true);
        assert_eq!(uf.connected(2, 3), true);
        assert_eq!(uf.connected(0, 3), false);

        uf.union(1, 2);
        assert_eq!(uf.count(), 1);
        assert_eq!(uf.connected(0, 3), true);
    }

    #[test]
    fn three_components() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(2, 4);

        assert_eq!(uf.count(), 3);

        uf.union(5, 0);
        assert_eq!(uf.count(), 2);
        assert_eq!(uf.connected(1, 5), true);
    }
}