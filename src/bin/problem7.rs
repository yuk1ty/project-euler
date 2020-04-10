fn main() {
    let n = 10_001;
    let mut ps = vec![2];
    let mut xs = vec![true; n / 2];
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

    println!("{}", ps.last().unwrap());
}
