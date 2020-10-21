// Bubble sort: run time O(n^2) worst case, O(n) in best case with early termination, space - O(1)
pub fn bubble<T>(list: &mut [T]) where T: std::cmp::PartialOrd {
    for i in 0..list.len()-1 {
        let mut swapped = false;
        for j in 0..list.len()-i-1 {
            if list[j+1] < list[j] {
                swapped = true;
                list.swap(j, j+1);
            }
        }
        if !swapped {
            // already sorted
            break;
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
    fn bubble_two_elements() {
        let mut input = vec![2, 1];
        bubble(&mut input);
        check_order(&input);
    }

    #[test]
    fn bubble_basic_sort() {
        let mut input = vec![1, 3, 2, 5, 4];
        bubble(&mut input);
        check_order(&input);
    }

    #[test]
    fn bubble_asc_sort() {
        let mut input = vec![1, 2, 3, 4, 5];
        bubble(&mut input);
        check_order(&input);
    }

    #[test]
    fn bubble_desc_sort() {
        let mut input = vec![5, 4, 3, 2, 1];
        bubble(&mut input);
        check_order(&input);
    }
}