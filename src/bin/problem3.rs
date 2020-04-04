fn main() {
    // brute_force can't over!
    println!("{}", biggest_prime_factor_brute_force(600851475143));
}

fn is_prime(n: i64) -> bool {
    if n == 0 || n == 1 || is_even(n) {
        return false;
    }

    for i in 3..((n as f64).sqrt() as i64) {
        if is_even(n) {
            continue;
        }
        if n as f64 % i as f64 == 0.0 {
            return false;
        }
    }
    true
}

fn is_even(n: i64) -> bool {
    n & 1 == 0
}

fn biggest_prime_factor_brute_force(n: i64) -> i64 {
    let mut biggest = 0;

    for i in 2..n {
        if n as f64 % i as f64 == 0.0 {
            if is_prime(i) {
                if i > biggest {
                    biggest = i;
                }
            }
        }
    }

    biggest
}
