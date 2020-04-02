fn main() {
    println!("{}", calculate_total_sum(1000));
}

fn calculate_total_sum(range: i32) -> i32 {
    let mut total_sum = 0;

    for i in 1..range {
        if i % 3 == 0 || i % 5 == 0 {
            total_sum += i;
        }
    }

    total_sum
}

#[test]
fn calculate_range_10() {
    assert_eq!(calculate_total_sum(10), 23);
}
