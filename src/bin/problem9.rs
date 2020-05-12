fn main() {
    let mut ans = 0;
    for n in 1..32 {
        for m in 1..n {
            let a = n * n - m * m;
            let b = 2 * n * m;
            let c = n * n + m * m;
            if a + b + c == 1000 {
                ans = a * b * c;
                break;
            }
        }
    }

    println!("{}", ans);
    println!("{}", another_ans());
}

fn another_ans() -> i32 {
    let mut ans = 0;
    for a in 1..1001 {
        for b in 1..1001 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                ans = a * b * c;
                break;
            }
        }
    }

    return ans;
}
