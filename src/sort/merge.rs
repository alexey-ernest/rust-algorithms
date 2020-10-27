/// Merge sort: best and worst run time complexity is O(n*logn), space - O(n)
pub fn merge<T>(list: &mut [T]) where T: std::cmp::PartialOrd + Copy {
    sort_internal(list, 0, list.len()-1);
}

fn sort_internal<T>(list: &mut [T], l: usize, r: usize) where T: std::cmp::PartialOrd + Copy {
    if l >= r {
        return;
    }

    let mid = (l+r)/2;
    sort_internal(list, l, mid);
    sort_internal(list, mid+1, r);
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
mod test {
    use super::*;

    fn check_order<T>(list: &[T]) where T: std::cmp::PartialOrd + std::fmt::Debug {
        let mut prev: &T = &list[0];
        for a in list.iter() {
            assert!(a >= prev, "non-decreasing order violation: {:?}", list);
            prev = a;
        }
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
