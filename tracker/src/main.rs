//! Finance Tracker in Rust!
//! 
//! Learning by doing is the best type of learning in my book!


mod tracker_features {
    pub mod expense_category;
}
use tracker_features::expense_category::ExpenseCategory;

use core::num;
use std::{io, num::ParseIntError};


fn welcome_message() {

    println!("\nWelcome to the finance tracker. You're going to be finacially smart!");
    println!("Please choose something from the options below.");
}


fn get_user_selection() -> String {

    println!("\nOptions are below");
    println!("1. Add a new transaction");
    println!("2. View transactions and summary within a date range");
    println!("3. Exit");

    let mut user_choice: String = String::new();


    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read the input!");

    return user_choice

}

fn check_user_input(user_choice: &String) -> Result<u32,ParseIntError> {

    let num_choice: Result<u32, ParseIntError> = user_choice.trim().parse();

    return num_choice
}


fn main() {
    
    welcome_message();

    loop {

        
        let num_choice: Result<u32, ParseIntError> = check_user_input(&user_choice);

        if num_choice.is_err() {

        }


    }
}
