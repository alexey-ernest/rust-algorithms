// Insertion sort: run time O(n^2) average case, O(n) best case for sorted slices, space - O(1)
pub fn insertion<T>(list: &mut [T]) where T: std::cmp::PartialOrd {
    for i in 1..list.len() {
        for j in (1..i+1).rev() {
            if list[j] < list[j-1] {
                list.swap(j, j-1);
            }
        }
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
    fn insertion_two_elements() {
        let mut input = vec![2, 1];
        insertion(&mut input);
        check_order(&input);
    }

    #[test]
    fn insertion_basic_sort() {
        let mut input = vec![1, 3, 2, 5, 4];
        insertion(&mut input);
        check_order(&input);
    }

    #[test]
    fn insertion_asc_sort() {
        let mut input = vec![1, 2, 3, 4, 5];
        insertion(&mut input);
        check_order(&input);
    }

    #[test]
    fn insertion_desc_sort() {
        let mut input = vec![5, 4, 3, 2, 1];
        insertion(&mut input);
        check_order(&input);
    }
}