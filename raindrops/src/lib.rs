pub fn raindrops(n: u32) -> String {
    let is_factor = |num: u32| n % num == 0;
    let mut result = String::from("");
    if is_factor(3) {
        result.push_str("Pling")
    }
    if is_factor(5) {
        result.push_str("Plang")
    }
    if is_factor(7) {
        result.push_str("Plong")
    }

    match result.len() {
        0 => n.to_string(),
        _ => result,
    }
}
