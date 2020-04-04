fn main() {
    // brute_force can't over!
    println!("{}", biggest_prime_factor_brute_force(600851475143));
}

fn is_prime(n: i64) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    let factors_table = factors(n);
    if factors_table.len() != 0 {
        return false;
    }

    true
}

fn factors(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    for i in 2..n {
        if n as f64 % i as f64 == 0.0 {
            factors.push(i);
        }
    }
    factors
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
