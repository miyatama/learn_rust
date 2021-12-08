use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guessing the number");

    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..101);

    loop {
        println!("input your number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
