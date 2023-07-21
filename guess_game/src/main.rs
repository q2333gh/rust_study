use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is :{}", secret_number); // {} in println!("{}") :  A placeholder for variables of consts.
    loop {
        let mut guess = String::new(); // mut : means it`s a variable . in rust every variable is immutable by default.
        io::stdin()
            .read_line(&mut guess)
            .expect("fail to read line"); // expect: force user to handle exception
                                          // reassinged guess: called shadowing, ussually use for type casting
        let guess: u32 = match guess.trim().parse() {
            // :u32 : type casting, work with .parse()
            Ok(num) => num,
            Err(_) => {
                //_  means any error
                println!("{}", "Please input numbers!".red());
                continue;
            }
        };

        println!("You guessed : {}", guess);
        // match : called the  magic expression
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
