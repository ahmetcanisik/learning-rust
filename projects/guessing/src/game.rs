#![allow(dead_code)]
#![allow(unused_variables)]

use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Write},
};

pub struct GuessingGame {
    pub game_name: String,
    pub guess_left: u8,
    pub alert: String,
    pub distance: (i32, i32),
}

impl GuessingGame {
    pub fn play(&mut self) -> &str {
        if self.alert == "test" {
            return "test passed!";
        }

        // Generating a random number with the rand library.
        let mut secret_number: i32 = rand::thread_rng().gen_range(self.distance.0..=self.distance.1); // equals(=) including 100 in the range function.

        loop {
            self.init();
    
            if self.guess_left == 0 {
                self.update(0, &format!("Not another your guess left! I'm sorry but you lost! 😭\nThe secret number is {secret_number}"));
                self.init();
                break "finished";
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
    
            self.guess_left -= 1;
    
            match guess.cmp(&mut secret_number) {
                Ordering::Equal => {
                    self.update(
                        self.guess_left,
                        &format!("Your guess {guess}, correct! and you win!"),
                    );
                    self.init();
                    break "finished";
                }
                Ordering::Less => {
                    self.update(
                        self.guess_left,
                        &format!("Your guess {guess}, less then the secret number."),
                    );
                }
                Ordering::Greater => {
                    self.update(
                        self.guess_left,
                        &format!("Your guess {guess}, greater then the secret number."),
                    );
                }
            }
        }
    }

    fn init(&self) {
        // clearing console....
        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().flush().unwrap();

        println!("\n\n━━━━━━━━━ {} ━━━━━━━━━\n", self.game_name);

        match self.guess_left {
            0 => println!("Game finished!"),
            guess => {
                for _ in 0..self.guess_left {
                    print!("❤");
                    io::stdout().flush().unwrap();
                }
                println!("\n");

                if guess == 1 {
                    println!("Last chance! Good use for you!");
                }
            }
        }

        if self.alert == "first" {
            println!("Welcome to {}\nThe guessing_game rules is very simple.\nGuess the secret number({} to {}) and you win!", self.game_name, self.distance.0, self.distance.1);
            println!("But dont't mistake your guess for unlimited!\n");
        } else {
            println!("{}\n", self.alert);
        }
    }

    fn update(&mut self, guess_left: u8, alert: &str) {
        self.guess_left = guess_left;
        self.alert = alert.to_string();
    }
}