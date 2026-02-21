use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::fs;
use rand::Rng;


const PB_FILE: &str = "pb.pf";
const PB_COUNT: usize = 3;

fn print_menu(pbs: [u32; 3]) {
    let fmt = |v: u32| if v == 0 { "None".to_string() } else { v.to_string() };

    println!("==============================");
    println!("PocketFriends.Org      v0.2.1");
    println!("==============================");
    println!(
        "PBs: Easy {} | Med {} | Hard {}",
        fmt(pbs[0]),
        fmt(pbs[1]),
        fmt(pbs[2]),
    );
    println!("==============================");
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
        let pbs = load_pbs().unwrap_or([0, 0, 0]);

        clearscreen::clear().expect("failed to clear screen");
        print_menu(pbs);

        let mut menu_choice = String::new();
        io::stdin()
            .read_line(&mut menu_choice)
            .expect("Failed to read your guess.");

        let menu_choice: u32 = match menu_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match menu_choice {
            1 => game(50),
            2 => game(100),
            3 => game(500),
            4 => break,
            _ => println!("Please choose one of the provided options."),
        }
    }
}

fn game(diff: u32) {
   clearscreen::clear().expect("failed to clear screen");
   println!("Guess the number!\n");

   let secret_number = rand::thread_rng().gen_range(1..=diff);

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
            if let Some(i) = diff_to_index(diff) {
                if let Err(e) = try_update_pb(i, guess_amount) {
                    eprintln!("Failed to save PB: {}", e);
                }
            }
            println!("You win, with {guess_amount} attempts!");
            println!("\nPress Enter to return to the menu...");
            let _ = std::io::stdin().read_line(&mut String::new());
            break;
        }
    }
   }
   return;
}

fn load_pbs() -> io::Result<[u32; 3]> {
    let bytes = match fs::read(PB_FILE) {
        Ok(b) => b,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok([0, 0, 0]),
        Err(e) => return Err(e),
    };

    if bytes.len() != 12 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} must be 12 bytes (3 u32), got {}", PB_FILE, bytes.len()),
        ));
    }

    let read_u32 = |i: usize| -> u32 {
        let o = i * 4;
        u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]])
    };

    Ok([read_u32(0), read_u32(1), read_u32(2)])
}

fn save_pbs(pbs: [u32; 3]) -> io::Result<()> {
    let mut out = Vec::with_capacity(12);
    for v in pbs {
        out.extend_from_slice(&v.to_le_bytes());
    }
    fs::write(PB_FILE, out)
}

fn try_update_pb(diff_index: usize, attempts: u32) -> io::Result<()> {
    let mut pbs = load_pbs()?;
    let cur = pbs[diff_index];

    if cur == 0 || attempts < cur {
        pbs[diff_index] = attempts;
        save_pbs(pbs)?;
    }
    Ok(())
}

fn diff_to_index(diff: u32) -> Option<usize> {
    match diff {
        50 => Some(0),
        100 => Some(1),
        500 => Some(2),
        _ => None,
    }
}