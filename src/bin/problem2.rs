fn main() {
    println!("{}", sum_even_valued_terms());
}

fn sum_even_valued_terms() -> usize {
    let mut sum = 0;
    let mut memo: [Option<usize>; 4000000] = [None; 4000000];
    memo[0] = Some(1);
    memo[1] = Some(1);

    for i in 2..40 {
        memo[i] = Some(memo[i - 1].unwrap() + memo[i - 2].unwrap());

        if memo[i].unwrap() % 2 == 0 {
            sum += memo[i].unwrap();
        }
    }

    sum
}
