use crate::structs::Heap;

// Heap sort: O(n*logn) run time complexity, O(1) - space complexity.
pub fn heap<T>(list: &mut [T]) where T: std::cmp::PartialOrd + Copy + Default {
    let mut h: Heap<T> = Heap::from(list);
    for e in list.iter_mut() {
        *e = h.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_order<T>(list: &[T]) where T: std::cmp::PartialOrd + std::fmt::Debug {
        let mut prev: &T = &list[0];
        for a in list.iter() {
            assert!(a >= prev, "non-decreasing order violation: {:?}", list);
            prev = a;
        }
    }


    #[test]
    fn heap_two_elements() {
        let mut input = vec![2, 1];
        heap(&mut input);
        check_order(&input);
    }

    #[test]
    fn heap_basic_sort() {
        let mut input = vec![1, 3, 2, 5, 4];
        heap(&mut input);
        check_order(&input);
    }

    #[test]
    fn heap_asc_sort() {
        let mut input = vec![1, 2, 3, 4, 5];
        heap(&mut input);
        check_order(&input);
    }

    #[test]
    fn heap_desc_sort() {
        let mut input = vec![5, 4, 3, 2, 1];
        heap(&mut input);
        check_order(&input);
    }    
}