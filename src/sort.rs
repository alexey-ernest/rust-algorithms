// Selection sort: run time O(n^2), space - O(1)
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

// Shell sort (improved insertion sort): run time O(n^2) average case, O(n) best case for sorted slices, space - O(1)
pub fn shell<T>(list: &mut [T]) where T: std::cmp::PartialOrd {
    let mut k = list.len() / 2;
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

// Merge sort: run time O(n*logn), space O(n)
pub fn merge<T>(list: &mut [T]) where T: std::cmp::PartialOrd + Copy {
    merge_sort(list, 0, list.len()-1);
}

fn merge_sort<T>(list: &mut [T], l: usize, r: usize) where T: std::cmp::PartialOrd + Copy {
    if l >= r {
        return;
    }

    let mid = (l+r)/2;
    merge_sort(list, l, mid);
    merge_sort(list, mid+1, r);
    merge_internal(list, l, mid, r);
}

fn merge_internal<T>(list: &mut [T], l: usize, mid: usize, r: usize) where T: std::cmp::PartialOrd + Copy {
    let mut copy = Vec::new();
    for i in l..r+1 {
        copy.push(list[i]);
    }

    let mid = mid-l;
    let r = r-l;
    let mut i = 0;
    let mut j = mid+1;
    let mut k = l;
    while i <= mid && j <= r {
        if copy[i] < copy[j] {
            list[k] = copy[i];
            i += 1;
        } else {
            list[k] = copy[j];
            j += 1;
        }
        k += 1;
    }

    // handling tail
    while i <= mid {
        list[k] = copy[i];
        i += 1;
        k += 1;
    }

    while j <= r {
        list[k] = copy[j];
        j += 1;
        k += 1;
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


    #[test]
    fn merge_two_elements() {
        let mut input = vec![2, 1];
        merge(&mut input);
        check_order(&input);
    }

    #[test]
    fn merge_basic_sort() {
        let mut input = vec![1, 3, 2, 5, 4];
        merge(&mut input);
        check_order(&input);
    }

    #[test]
    fn merge_asc_sort() {
        let mut input = vec![1, 2, 3, 4, 5];
        merge(&mut input);
        check_order(&input);
    }

    #[test]
    fn merge_desc_sort() {
        let mut input = vec![5, 4, 3, 2, 1];
        merge(&mut input);
        check_order(&input);
    }
}
