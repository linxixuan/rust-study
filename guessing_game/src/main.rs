use rand::Rng;

fn main() {
    println!("Guess the number !");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    std::io::stdin()
        .read_line(&mut guess) // need referenc use same mutable prefix
        .expect("Failed to read line");
        // The right way to suppress the warning is to actually write error handling, but in our case we just want to crash this program when a problem occurs, so we can use expect. You’ll learn about recovering from errors in Chapter 9.

    println!("You guessed: {guess}");
}
