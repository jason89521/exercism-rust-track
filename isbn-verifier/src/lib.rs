/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // The iter now doesn't contain the '-'.
    let mut isbn = isbn.chars().filter(|ch| ch != &'-');
    if isbn.clone().count() != 10 {
        return false;
    }

    let mut coefficient = 10;
    let result = isbn.try_fold(0u32, |acc, ch| {
        if ch == 'X' {
            if coefficient != 1 {
                return None;
            }
            return Some(acc + 10);
        } else {
            match ch.to_digit(10) {
                Some(digit) => {
                    coefficient -= 1;
                    Some(acc + (coefficient + 1) * digit)
                }
                None => None,
            }
        }
    });

    match result {
        Some(result) => (result % 11) == 0,
        None => false,
    }
}
