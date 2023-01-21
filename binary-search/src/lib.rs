use std::cmp::Ordering;

pub fn find<T: Ord, R: AsRef<[T]>>(array: R, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut start = 0;
    let mut end = array.len();
    let mut middle = end / 2;
    while middle >= start && middle < end {
        match key.cmp(&array[middle]) {
            Ordering::Equal => return Some(middle),
            Ordering::Less => {
                end = middle;
            }
            Ordering::Greater => start = middle + 1,
        }
        middle = (start + end) / 2;
    }

    None
}
