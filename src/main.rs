use std::cmp::Ordering;
use std::io;
use std::io::Write;

use rand::Rng;

fn print_menu() {
    println!("Choose your difficulty.");
    println!("1. Easy (1-50)");
    println!("2. Medium (1-100)");
    println!("3. Hard (1-500)");
    println!("4. Exit Game");
    print!("> ");
    io::stdout().flush().ok();
}

fn main() {
    loop {
        print_menu();

        let mut menu_choice = String::new();
        io::stdin()
            .read_line(&mut menu_choice)
            .expect("Failed to read your guess.");

        let menu_choice: u32 = match menu_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match menu_choice {
            1 => easy(),
            2 => medium(),
            3 => hard(),
            4 => break,
            _ => println!("Please choose one of the provided options."),
        }
    }
}

fn easy() {
   println!("Guess the number!");

   let secret_number = rand::thread_rng().gen_range(1..=50);

   let mut guess_amount = 0;

   loop {
    println!("Please input your guess.");
    print!("> ");
    io::stdout().flush().ok();

    let mut guess = String::new();

    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    guess_amount += 1;

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win, with {guess_amount} attempts!");
            break;
        }
    }
   }
   return;
}

fn medium() {
   println!("Guess the number!");

   let secret_number = rand::thread_rng().gen_range(1..=100);

   let mut guess_amount = 0;

   loop {
    println!("Please input your guess.");
    print!("> ");
    io::stdout().flush().ok();

    let mut guess = String::new();

    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    guess_amount += 1;

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win, with {guess_amount} attempts!");
            break;
        }
    }
   }
   return;
}

fn hard() {
   println!("Guess the number!");

   let secret_number = rand::thread_rng().gen_range(1..=500);

   let mut guess_amount = 0;

   loop {
    println!("Please input your guess.");
    print!("> ");
    io::stdout().flush().ok();

    let mut guess = String::new();

    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    guess_amount += 1;

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win, with {guess_amount} attempts!");
            break;
        }
    }
   }
   return();
}