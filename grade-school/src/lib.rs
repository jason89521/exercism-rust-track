use std::collections::HashMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School<'a> {
    map: HashMap<u32, Vec<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        self.map
            .entry(grade)
            .and_modify(|map| map.push(student))
            .or_insert(vec![student]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result: Vec<u32> = self.map.keys().map(|k| *k).collect();
        result.sort();

        result
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.map.get(&grade) {
            Some(slice) => {
                let mut result: Vec<String> = slice.iter().map(|&str| str.to_owned()).collect();
                result.sort();

                result
            }
            None => vec![],
        }
    }
}
