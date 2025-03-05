//! Finance Tracker in Rust!
//! 
//! Learning by doing is the best type of learning in my book!

mod tests;



mod user_features {
    pub mod user_options;
}

use user_features::user_options::check_user_input;




pub fn welcome_message() {

    println!("\nWelcome to the finance tracker. You're going to be finacially smart!");
    println!("Please choose something from the options below.");
}

pub fn add(left: usize, right:usize) -> usize {
    left + right
}



fn main() {
    
    welcome_message();

    let input: String = String::from("hiii");

    match check_user_input(&input) {
        Ok(num) =>  println!("Success"),
        Err(e) => println!("Naaa")
    }
}
