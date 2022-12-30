/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|ch| !ch.is_whitespace())
        .rev()
        .try_fold((0u32, 0u32), |(sum, count), ch| {
            ch.to_digit(10)
                .map(|num| if count % 2 == 0 { num } else { num * 2 })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (sum + num, count + 1))
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}
