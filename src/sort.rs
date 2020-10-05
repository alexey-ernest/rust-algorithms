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
        for j in (0..i+1).rev() {
            if list[j] < list[i] {
                list.swap(i, j);
            }
        }
    }
}