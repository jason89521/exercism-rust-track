// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();
    for word in magazine {
        match map.get(word) {
            Some(n) => map.insert(word, n + 1),
            None => map.insert(word, 1),
        };
    }

    for word in note {
        match map.get(word) {
            Some(&n) => {
                if (n > 0) {
                    map.insert(word, n - 1);
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}
