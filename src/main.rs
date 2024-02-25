use std::io;

fn main() {
    println!("--- sum to n ---");
    println!("Please enter a number :");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read number");

    let number: i32 = n.trim().parse().expect("Please enter a number");

    let sum = sum_to_n(number);

    println!("the sum of numbers from 0 to {} is : {}", number, sum);
}

fn sum_to_n(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
    }
    sum
}
