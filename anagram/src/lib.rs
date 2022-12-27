use std::collections::{HashMap, HashSet};

fn create_chars_map(str: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for char in str.chars() {
        map.entry(char).and_modify(|e| *e += 1).or_insert(1);
    }
    map
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set: HashSet<&'a str> = HashSet::new();
    let word = word.to_lowercase();
    let word_len = word.len();
    let word_map: HashMap<char, u32> = create_chars_map(&word);
    for &candidate in possible_anagrams {
        if candidate.len() != word_len {
            continue;
        }

        let candidate_lowercase = candidate.to_lowercase();
        if candidate_lowercase == word {
            continue;
        }

        if create_chars_map(&candidate_lowercase) == word_map {
            set.insert(candidate);
        }
    }

    set
}
