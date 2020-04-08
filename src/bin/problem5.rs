fn main() {
    let mut common_multiple = 1;
    for i in 1..21 {
        let new_common_multiple = lcm(common_multiple, i);
        common_multiple = new_common_multiple
    }
    println!("{}", common_multiple);
}

fn gcd(a: i32, b: i32) -> i32 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}
