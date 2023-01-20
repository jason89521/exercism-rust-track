use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let numbers = parse(input);
    let char_set = numbers.iter().flat_map(|s| s.chars()).collect();
    let is_zero_map = get_is_zero_map(&numbers);

    return recursive(&char_set, &numbers, &is_zero_map, &[]);
}

/// Return an array whose last element is the sum, and the other elements are the operands.
fn parse(input: &str) -> Vec<&str> {
    let parsed: Vec<&str> = input.split("==").collect();
    let sum = parsed[1].trim();
    let mut operands: Vec<&str> = parsed[0].split("+").map(|op| op.trim()).collect();
    operands.push(sum);
    operands
}

/// If the value is `true` then the key shouldn't be a zero.
fn get_is_zero_map(array: &[&str]) -> HashMap<char, bool> {
    let mut map = HashMap::new();
    for op in array.iter() {
        let mut is_first = true;
        for c in op.chars() {
            if is_first {
                map.insert(c, true);
                is_first = false;
            } else {
                map.entry(c).or_insert(false);
            }
        }
    }
    map
}

fn recursive(
    char_set: &HashSet<char>,
    numbers: &[&str],
    is_zero_map: &HashMap<char, bool>,
    permutation: &[u8],
) -> Option<HashMap<char, u8>> {
    if char_set.len() == permutation.len() {
        let solution: HashMap<char, u8> = char_set
            .iter()
            .cloned()
            .zip(permutation.iter().cloned())
            .collect();
        if check(&solution, numbers, is_zero_map) {
            return Some(solution);
        } else {
            return None;
        }
    }

    for i in 0..=9 {
        if !permutation.contains(&i) {
            if let Some(solution) = recursive(
                char_set,
                numbers,
                is_zero_map,
                &[permutation, &[i]].concat(),
            ) {
                return Some(solution);
            }
        }
    }

    None
}

fn check(
    solution: &HashMap<char, u8>,
    numbers: &[&str],
    is_zero_map: &HashMap<char, bool>,
) -> bool {
    if is_zero_map
        .iter()
        .filter(|&(_, &is_zero)| is_zero)
        .any(|(c, _)| solution[c] == 0)
    {
        return false;
    }

    let numbers = numbers
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| solution[&c].to_string())
                .collect::<String>()
        })
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    if numbers[0..numbers.len() - 1].iter().sum::<u64>() == numbers[numbers.len() - 1] {
        return true;
    } else {
        return false;
    }
}
