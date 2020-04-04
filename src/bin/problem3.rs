fn main() {
    println!("{}", biggest_prime_factor(600851475143));
}

fn biggest_prime_factor(m: i64) -> i64 {
    let mut n: i64 = m;
    for p in 2..n {
        if p * p > n {
            break;
        } else if n % p == 0 {
            n /= p;
            while n % p == 0 {
                n /= p;
            }
            if n == 1 {
                return p;
            }
        }
    }
    return n;
}
