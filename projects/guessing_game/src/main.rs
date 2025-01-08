use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

fn guessing_game(guess_left: u8, alert: &str) {
    clear_console();
    let game_name = "Canisik Guessing Game!";

    println!("\n\n━━━━━━━━━ {game_name} ━━━━━━━━━\n");

    match guess_left {
        0 => println!("Game finished!"),
        guess => {
            for _ in 0..guess_left {
                print!("❤");
                io::stdout().flush().unwrap();
            }
            println!("\n");

            if guess == 1 {
                println!("Last chance! Good use for you!");
            }
        }
    }

    if alert == "first" {
        println!("Welcome to {game_name}\nThe game rules is very simple. Guess the secret number and you win!");
        println!("But dont't mistake your guess for unlimited!\n");
    } else {
        println!("{alert}\n");
    }
}

fn main() {
    // Generating a random number with the rand library.
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100); // equals(=) including 100 in the range function.

    let mut user_guess: u8 = 5;

    guessing_game(user_guess, "first");

    loop {
        if user_guess == 0 {
            guessing_game(user_guess, &format!("Not another your guess left! I'm sorry but you lost! 😭\nThe secret number is {secret_number}"));
            break;
        }

        // Defining a variable to store the user's guess.
        let mut guess = String::new();

        // We are saying provide a guess for user.
        print!("━━━> Please provide your guess: ");
        io::stdout()
            .flush()
            .expect("Something went wrong when asking the user to provide a guess.");

        // We are asking guess for the user's and storing it in the guess variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong when asking for the user's guess.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        user_guess -= 1;

        match guess.cmp(&mut (secret_number as i32)) {
            Ordering::Equal => {
                guessing_game(user_guess, &format!("Your guess {guess}, correct! and you win!"));
                break;
            }
            Ordering::Less => {
                guessing_game(user_guess, &format!("Your guess {guess}, less then the secret number."));
            }
            Ordering::Greater => {
                guessing_game(user_guess, &format!("Your guess {guess}, greater then the secret number."));
            }
        }
    }
}