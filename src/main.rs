use std::io; // For input/output ...
use rand::Rng; // Imports random number functionality ...
use std::cmp::Ordering; // For comparing ...
use colored::*; // For colors ...

fn main() {
    let mut high_score = 0; // Track best score across games ...

    // Outer loop for playing multiple games ...
    loop { 
    println!("\n=== {} ===", "🎲 Welcome to the Number Guessing Game! 🎲".bright_cyan().bold());
    println!("{}", "You have 3 attempts to guess a number between 1 and 100!".yellow());

    if high_score > 0 {
        println!("{} {}", "Best Score:".bright_purple(), high_score.to_string().bright_purple().bold());
    }

    // This generates a random number ...
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    // Add an attempts counter ...
    let mut attempts_left = 3;

    // Track the previous guess ...
    let mut last_guess: Option<u32> = None;

    // Inner loop for guesses ...
        loop {
            println!("\nYou have {}/3 attempts left.", attempts_left.to_string().bright_green());
            println!("Please input your guess:");

            let mut guess = String::new(); // Creates a new empty string ...
            io::stdin()
                .read_line(&mut guess)  // Reads a line of input and stores it in the 'guess' variable ...
                .expect("Failed to read line");

            // Converts String to number
            let guess: u32 = match guess.trim().parse() {
                Ok(num) if num >=1 && num <= 100 => num,
                Ok(_) => {
                    println!("{}", "Please enter a number between 1 and 100!".red().bold());
                    continue;
                }
                Err(_) => {
                    println!("{}", "That's not a valid number!".red().bold());
                    continue;
                }
            };

            // Hot/Cold hint system ...
            if let Some(last) = last_guess {
                let last_diff = (secret_number as i32 - last as i32).abs();
                let current_diff = (secret_number as i32 - guess as i32).abs();
                let hint = if current_diff < last_diff {
                    "🔥 Getting Hotter! 🔥".bright_red()
                } else if current_diff > last_diff {
                    "❄️ Getting Colder! ❄️".bright_blue()
                } else {
                    "🌡️ Same Temperature! 🌡️".yellow()
                };
                println!("{}", hint);
            }
            last_guess = Some(guess);

            println!("You guessed: {}", guess.to_string().yellow());

            // The comparison logic ...
            match guess.cmp(&secret_number) {
                Ordering::Less => print!("{}", "🔽 Too small! ".bright_blue()),
                Ordering::Greater => print!("{}", "🔼 Too big! ".bright_blue()),
                Ordering::Equal => {
                    let score = attempts_left * 100;
                    print!("\n{}", "🎉 Congratulations! You win! 🎉".bright_green().bold());
                    println!("{} {}", "Score:".bright_yellow(), score.to_string().bold());

                    if score > high_score {
                        high_score = score;
                        println!("{}", "🏆 New High Score! 🏆".bright_purple().bold());
                    }
                    break;
                }
            }

            // Decrease attempts and check if we're out ...
            attempts_left -= 1;
            if attempts_left == 0 {
                println!("{}", "\n === 😢 Game Over! You're out of attempts. ===".bright_red().bold());
                println!("The number was: {}", secret_number.to_string().bright_yellow());
                break;
            }
        }

        // Ask to play again
        println!("\n{}", "::: Would you like to play again? (y/n): :::".cyan());
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() !="y" {
            println!("\n{}", "Thanks for playing! Goodbye! 👋".bright_green().bold());
            break;
        }
    }

}
