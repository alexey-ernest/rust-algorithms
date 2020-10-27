/// Dijkstra flag sort algorithm. Sorts a list of only 3 values {0, 1, 2} in O(n) without additional memory usage.
pub fn flag(list: &mut [i32]) {    
    let mut l = 0;
    let mut i = 0;
    let mut g = list.len()-1;
    let p = 1;
    while i < g {
        if list[i] < p {
            list.swap(l, i);
            l += 1;
            i += 1;
        } else if list[i] > p {
            list.swap(i, g);
            g -= 1;
        } else {
            i += 1;    
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_order<T>(list: &[T]) where T: PartialOrd + std::fmt::Debug {
        let mut prev = &list[0];
        for e in list.iter() {
            assert!(e >= prev, "non-decreasing order vialation for {:?}", list);
            prev = e;
        }
    }

    #[test]
    fn sort_3_elements() {
        let mut input = vec![0, 1, 2];
        flag(&mut input);
        check_order(&input);
    }

    #[test]
    fn sort_3_elements_desc() {
        let mut input = vec![2, 1, 0];
        flag(&mut input);
        check_order(&input);
    }

    #[test]
    fn sort_3_values() {
        let mut input = vec![2, 1, 0, 0, 0, 2, 2, 2, 1, 2, 1, 2, 1, 2, 0, 2, 1, 2, 0];
        flag(&mut input);
        check_order(&input);
    }

    #[test]
    fn sort_2_values_01() {
        let mut input = vec![0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1];
        flag(&mut input);
        check_order(&input);
    }

    #[test]
    fn sort_2_values_02() {
        let mut input = vec![2, 0, 0, 2, 2, 0, 0, 2, 0, 2, 0, 2, 2, 2];
        flag(&mut input);
        check_order(&input);
    }

    #[test]
    fn sort_2_values_12() {
        let mut input = vec![2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1];
        flag(&mut input);
        check_order(&input);
    }
}