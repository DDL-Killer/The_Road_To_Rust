use std::cmp::Ordering;
use std::io;
use rand::Rng;
use clearscreen::clear;

//1 -- 100,rand(crate)
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut last_result = String::new();
    loop {
        //加入清屏
        clear().expect("failed to clear screen");
        if !last_result.is_empty() {
            println!("{}", last_result);
            println!("--------------------------------");
        }
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            // Result
            // -- OK(usize)
            // -- Err
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                last_result = String::from("Please enter a valid number!");
                continue;
            },
        };
        //shadowing  遮蔽
        // println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => last_result = format!("You guessed: {}\nToo small!", guess),
            Ordering::Greater => last_result = format!("You guessed: {}\nToo big!", guess),
            Ordering::Equal => {
                println!("The secret number is: {}", secret_number);
                println!("You win!");
                break;
            },
        }
    }
}