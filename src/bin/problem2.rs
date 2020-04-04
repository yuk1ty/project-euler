fn main() {
    println!("{}", sum_even_valued_terms());
    println!("{}", sum_even_valued_terms_with_memoize());
}

const SIZE: usize = 10;

fn sum_even_valued_terms() -> usize {
    let mut fib_sum = 0;
    let mut fib_current = 0;
    let mut fib_next = 1;

    for _ in 1..SIZE {
        let fib_total = fib_current + fib_next;
        fib_current = fib_next;
        fib_next = fib_total;

        if fib_total % 2 == 0 {
            fib_sum += fib_total;
        }
    }

    fib_sum
}

fn sum_even_valued_terms_with_memoize() -> usize {
    let mut sum = 0;
    let mut memo: [Option<usize>; SIZE] = [None; SIZE];
    memo[0] = Some(1);
    memo[1] = Some(1);

    for i in 2..SIZE {
        memo[i] = Some(memo[i - 1].unwrap() + memo[i - 2].unwrap());

        if memo[i].unwrap() % 2 == 0 {
            sum += memo[i].unwrap();
        }
    }

    sum
}
