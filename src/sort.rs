pub fn selection<T>(list: &mut [T]) where T: std::cmp::PartialOrd {
    for i in 0..list.len() {
        let mut mini = i;
        for j in i+1..list.len() {
            if list[j] < list[i] {
                mini = j
            }
        }
        list.swap(i, mini);
    }
}

pub fn insertion<T>(list: &mut [T]) where T: std::cmp::PartialOrd {
    for i in 0..list.len() {
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
            assert!(a >= prev, "ascending order violation: {:?}", list);
            prev = a;
        }
    }

    #[test]
    fn selection_two_elements() {
        let mut input = vec![2, 1];
        selection(&mut input);
        check_order(&input);
    }

    #[test]
    fn selection_basic_sort() {
        let mut input = vec![1, 3, 2, 5, 4];
        selection(&mut input);
        check_order(&input);
    }

    #[test]
    fn selection_asc_sort() {
        let mut input = vec![1, 2, 3, 4, 5];
        selection(&mut input);
        check_order(&input);
    }

    #[test]
    fn selection_desc_sort() {
        let mut input = vec![5, 4, 3, 2, 1];
        selection(&mut input);
        check_order(&input);
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
