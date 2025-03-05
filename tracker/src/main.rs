//! Finance Tracker in Rust!
//! 
//! Learning by doing is the best type of learning in my book!


mod tracker_features {
    pub mod expense_category;
}

mod user_features {
    pub mod user_options;
}

use user_features::user_options;
use tracker_features::expense_category::ExpenseCategory;

use core::num;
use std::{io::{self, Write}, num::ParseIntError};


pub fn welcome_message() {

    println!("\nWelcome to the finance tracker. You're going to be finacially smart!");
    println!("Please choose something from the options below.");
}





fn main() {
    
    welcome_message();

    loop {

        user_choice
        
        let num_choice: Result<u32, ParseIntError> = user_options::check_user_input(&user_choice);

        if num_choice.is_err() {

        }


    }
}
