pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];

    (2u32..)
        .filter(|&candidate| {
            if !primes.iter().any(|i| candidate % i == 0) {
                primes.push(candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}
