use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guess();
}

fn guess() {
    println!("Guess the number!!!");
    println!("Please enter your full name: ");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to get name!!!");
    name = name.trim().to_string();

    println!("Full name: {}", name);

    let first_name = first_word(&name);

    println!("First name: {}", first_name);

    let rand_num = rand::thread_rng().gen_range(1, 101);

    println!("Please input a number: ");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = trim_guess(guess);

        let guess: u32 = guess.parse().expect("Please enter a number!!!!");

        println!("You guessed: {}", guess);

        let test_var = 0;

        println!("Test Var = {}", test_var);

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn trim_guess(guess: String) -> String {
    guess.trim().to_string()
}

fn first_word(full: &String) -> String {
    for (ind, c) in full.char_indices() {
        if c == ' ' {
            return full[0..ind].to_string();
        }
    }
    full.to_string()
}
