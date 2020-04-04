use std::cmp::max;

fn main() {
    println!("{}", simple_solution());
}

fn simple_solution() -> i32 {
    let mut biggest = 0;
    for i in (100..1000).rev() {
        for j in (i..1000).rev() {
            let num = i * j;
            if is_palindrome(num) {
                biggest = max(biggest, num);
            }
        }
    }
    biggest
}

fn is_palindrome(n: i32) -> bool {
    let num_str = n.to_string();
    let reversed_num_str = num_str.chars().rev().collect::<String>();
    num_str == reversed_num_str
}
