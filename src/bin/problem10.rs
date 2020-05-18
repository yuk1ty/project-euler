fn main() {
    let ans: usize = sieve(2_000_000).iter().sum();
    println!("{}", ans);
}

fn sieve(n: usize) -> Vec<usize> {
    let mut ps = vec![2];
    let mut xs = vec![true; n / 2]; // 素数は確実に奇数なので、総数は半分以下でよくなる
    let mut x = 3;

    while x * x <= n {
        let mut y = (x - 3) / 2;
        if xs[y] {
            ps.push(x);
            y += x;
            while y < xs.len() {
                xs[y] = false;
                y += x;
            }
        }
        x += 2;
    }

    while x <= n {
        if xs[(x - 3) / 2] {
            ps.push(x);
        }
        x += 2;
    }
    ps
}
