use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => "".to_string(),
        Some(first_word) => list
            .windows(2)
            .map(|words| format!("For want of a {} the {} was lost.\n", words[0], words[1]))
            .chain(once(format!("And all for the want of a {}.", first_word)))
            .collect::<String>(),
    }
}
