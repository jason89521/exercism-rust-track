const PAIRS: [(char, char); 3] = [('{', '}'), ('[', ']'), ('(', ')')];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            '}' | ']' | ')' => match stack.pop() {
                Some(left) => {
                    if !PAIRS.contains(&(left, c)) {
                        return false;
                    }
                }
                None => return false,
            },
            _ => {}
        }
    }

    stack.is_empty()
}
