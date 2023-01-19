use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}

fn mod_exp(b: u64, e: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut result = 1;
    let mut e = e as u128;
    let mut b = b as u128;
    let m = m as u128;
    b = b % m;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % m;
        }
        e = e >> 1;
        b = (b * b) % m
    }

    result as u64
}
