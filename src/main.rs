use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game");
    println!("Hint: use divide and conqure algorithm to guess the number");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("secret number is: {secret_num}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Less!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Congrats! You WIN!");
                break;
            }
        }
    }
}
