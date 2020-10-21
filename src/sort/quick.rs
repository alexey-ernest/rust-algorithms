use rand::Rng;

// Quick sort: average run time O(n*logn), space - O(1)
pub fn quick<T>(list: &mut [T]) where T: std::cmp::PartialOrd {
    quick_internal(list, 0, list.len()-1);
}

fn quick_internal<T>(list: &mut [T], lo: usize, hi: usize) where T: std::cmp::PartialOrd {
    if lo >= hi {
        return;
    }

    let p = quick_partition(list, lo, hi);
    if p > 0 {
        quick_internal(list, lo, p-1);    
    }
    quick_internal(list, p+1, hi);
}

fn quick_partition<T>(list: &mut [T], lo: usize, hi: usize) -> usize 
    where T: std::cmp::PartialOrd {
    let mut rng = rand::thread_rng();
    let pivot_i = lo + rng.gen_range(0, hi-lo+1);
    list.swap(pivot_i, hi);
    
    let mut i = lo;
    for j in lo..hi {
        if list[j] < list[hi] {
            list.swap(i, j);
            i += 1;
        }
    }
    list.swap(i, hi);
    return i;
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
    fn quick_two_elements() {
        let mut input = vec![2, 1];
        quick(&mut input);
        check_order(&input);
    }

    #[test]
    fn quick_basic_sort() {
        let mut input = vec![1, 3, 2, 5, 4];
        quick(&mut input);
        check_order(&input);
    }

    #[test]
    fn quick_asc_sort() {
        let mut input = vec![1, 2, 3, 4, 5];
        quick(&mut input);
        check_order(&input);
    }

    #[test]
    fn quick_desc_sort() {
        let mut input = vec![5, 4, 3, 2, 1];
        quick(&mut input);
        check_order(&input);
    }    
}
