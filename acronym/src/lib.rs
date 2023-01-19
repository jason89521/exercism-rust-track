pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '_' || c == '-')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_ascii_uppercase())
                    .filter(|c| c.is_ascii_uppercase()),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
