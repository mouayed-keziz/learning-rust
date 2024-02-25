use rand::Rng;
use std::io;
fn main() {
    println!("--- Guessing game ---");
    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut number: i32 = 0;
    read_number(&mut number);
    let mut i = 0;
    loop {
        i += 1;
        if number == secret_number {
            println!("You guessed the secret number, its {}", secret_number);
            break;
        }
        if number < secret_number {
            println!("The secret number is greater than {}", number);
            read_number(&mut number)
        } else {
            println!("The secret number is lower than {}", number);
            read_number(&mut number)
        }
    }

    println!(
        "The secret number is: {}, number of tries {}",
        secret_number, i
    );
}

fn read_number(number: &mut i32) {
    println!("Guess the secret number");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read number");
    *number = guess.trim().parse().expect("Please enter a valid number");
}
