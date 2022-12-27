#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let is_sublist = first_list.len() == 0
        || second_list
            .windows(first_list.len())
            .any(|slice| slice == first_list);
    let is_superlist = second_list.len() == 0
        || first_list
            .windows(second_list.len())
            .any(|slice| slice == second_list);

    match (is_sublist, is_superlist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
