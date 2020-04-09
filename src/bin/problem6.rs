fn main() {
    let mut each_pow_sum: i64 = 0;
    let mut sum: i64 = 0;

    for i in 1..101 {
        each_pow_sum += i * i;
        sum += i;
    }

    let sum_pow = sum * sum;

    println!("each: {}", each_pow_sum);
    println!("sum: {}", sum_pow);

    let ans = abs(sum_pow - each_pow_sum);
    println!("{}", ans);
}

const fn abs(v: i64) -> i64 {
    [-v, v][(v >= 0) as usize]
}
