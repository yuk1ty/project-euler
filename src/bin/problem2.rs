struct Fibonacci {
    a: i64,
    b: i64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let x = self.a;
        self.a = self.b;
        self.b += x;
        Some(x)
    }
}

fn main() {
    let s: i64 = Fibonacci::new()
        .filter(|&f| f % 2 == 0)
        .take_while(|&f| f <= 4_000_000)
        .sum();
    println!("{}", s);
}
