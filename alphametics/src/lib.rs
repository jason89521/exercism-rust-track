use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let values = parse(input);
    let mut permutation = vec![0; values.len()];
    return recursive(&mut permutation, &values, &(0..10).collect::<Vec<_>>());
}

fn parse(input: &str) -> Vec<(char, i64, bool)> {
    let mut map = HashMap::new();
    let mut leadings = HashSet::new();
    let mut prev_char = ' ';
    // To total number is negative, such that we can examine the permutation
    // by checking whether the sum is zero.
    let mut weight = -1;
    input.chars().rev().for_each(|c| {
        if c.is_alphabetic() {
            *map.entry(c).or_insert(0) += weight;
            weight *= 10;
            prev_char = c;
        } else {
            weight = 1;
            leadings.insert(prev_char);
        }
    });
    leadings.insert(prev_char);
    map.iter()
        .map(|(&ch, &v)| (ch, v, leadings.contains(&ch)))
        .collect()
}

fn recursive(
    permutation: &mut [u8],
    values: &[(char, i64, bool)],
    unused_digits: &[u8],
) -> Option<HashMap<char, u8>> {
    // This index is shared by the `permutation` and `values`
    let value_index = 10 - unused_digits.len();
    if value_index == values.len() {
        let mut sum = 0;
        for (index, &digit) in permutation.iter().enumerate() {
            sum += digit as i64 * values[index].1;
        }
        if sum == 0 {
            let solution = permutation
                .iter()
                .zip(values.iter())
                .map(|(&digit, &(ch, _, _))| (ch, digit))
                .collect();
            return Some(solution);
        } else {
            return None;
        }
    }
    for (index, &digit) in unused_digits.iter().enumerate() {
        if digit == 0 && values[value_index].2 {
            continue;
        }

        permutation[value_index] = digit;
        let mut unused_digits = unused_digits.to_vec();
        unused_digits.remove(index);
        if let Some(solution) = recursive(permutation, values, &unused_digits) {
            return Some(solution);
        }
    }

    None
}
