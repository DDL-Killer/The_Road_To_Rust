use std::io::stdin;

//Prelude
fn main() {
    let mut msg:String = String::new();
    println!("Enter your message:");
    stdin().read_line(&mut msg).unwrap();
    println!("Message is {}", msg);
}
// crate
// -- library crate(1)
// -- binary crate(n)