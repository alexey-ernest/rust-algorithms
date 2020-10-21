// Shell sort (improved insertion sort): run time O(n^2) average case, O(n) best case for sorted slices, space - O(1)
pub fn shell<T>(list: &mut [T]) where T: std::cmp::PartialOrd {
    let mut k = list.len()/2;
    while k > 0 {
        let mut i = 0;
        while i < list.len() {
            let mut j = i;
            while j >= k {
                if list[j] < list[j-k] {
                    list.swap(j, j-k);
                }
                j -= k;
            }
            i += k;
        }
        k /= 2;
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
    fn shell_two_elements() {
        let mut input = vec![2, 1];
        shell(&mut input);
        check_order(&input);
    }

    #[test]
    fn shell_basic_sort() {
        let mut input = vec![1, 3, 2, 5, 4];
        shell(&mut input);
        check_order(&input);
    }

    #[test]
    fn shell_asc_sort() {
        let mut input = vec![1, 2, 3, 4, 5];
        shell(&mut input);
        check_order(&input);
    }

    #[test]
    fn shell_desc_sort() {
        let mut input = vec![5, 4, 3, 2, 1];
        shell(&mut input);
        check_order(&input);
    }    
}