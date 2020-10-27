/// Quick sort (3-way edition for inputs with many repeated elements): O(n*logn)
pub fn quick_3_way<T>(list: &mut [T]) where T: PartialOrd + Copy {
    quick_internal(list, 0, list.len()-1);
}

fn quick_internal<T>(list: &mut [T], lo: usize, hi: usize) where T: PartialOrd + Copy {
    if lo >= hi || hi >= list.len() {
        return;
    }

    let mut l = lo;
    let mut g = hi;
    let mut i = lo;
    let p = list[i];
    while i <= g {
        if list[i] < p {
            list.swap(l, i);
            l += 1;
        } else if list[i] > p {
            list.swap(i, g);
            g -= 1;
            i -= 1;
        }
        i += 1;
    }

    // excluding list[l..g+1] where all elements equal to p
    if l > lo {
        quick_internal(list, lo, l-1);    
    }
    quick_internal(list, g+1, hi);
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_order<T>(list: & [T]) where T: PartialOrd + std::fmt::Debug {
        let mut prev = &list[0];
        for e in list.iter() {
            assert!(e >= prev, "non-decreasing order violation: ${:?}", list);
            prev = e;
        }
    }

    #[test]
    fn quick3_two_elements() {
        let mut input = vec![2, 1];
        quick_3_way(&mut input);
        check_order(&input);
    }

    #[test]
    fn quick3_three_elements() {
        let mut input = vec![2, 1, 3];
        quick_3_way(&mut input);
        check_order(&input);
    }

    #[test]
    fn quick3_three_elements_desc() {
        let mut input = vec![3, 2, 1];
        quick_3_way(&mut input);
        check_order(&input);
    }

    #[test]
    fn quick3_asc_sort() {
        let mut input = vec![1, 2, 3, 4, 5];
        quick_3_way(&mut input);
        check_order(&input);
    }

    #[test]
    fn quick3_desc_sort() {
        let mut input = vec![5, 4, 3, 2, 1];
        quick_3_way(&mut input);
        check_order(&input);
    }

    #[test]
    fn quick3_repeated_elements() {
        let mut input = vec![2, 2, 1, 2, 2, 2, 0, 0, 1, 1, 0, 2, 1, 0, 2, 1, 1, 1, 0, 0, 2, 0, 2];
        quick_3_way(&mut input);
        check_order(&input);
    }
}