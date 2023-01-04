pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    for divisor in 2..n + 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        if n == 1 {
            break;
        }
    }

    factors
}
