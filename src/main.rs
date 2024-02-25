fn main() {
    let n = 12;
    println!("-----------------");
    for i in 1..n + 1 {
        print_multiplication_table(i, n);
        println!("-----------------");
    }
}

fn print_multiplication_table(i: i32, n: i32) {
    for j in 1..n + 1 {
        println!("{} x {} = {}", i, j, i * j);
    }
}
