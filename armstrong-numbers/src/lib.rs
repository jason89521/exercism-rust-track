pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let num_of_digits = str.len();
    str.chars()
        .try_fold(0, |acc, ch| {
            ch.to_digit(10)
                .unwrap()
                .pow(num_of_digits as u32)
                .checked_add(acc)
        })
        .map_or(false, |sum| sum == num)
}
