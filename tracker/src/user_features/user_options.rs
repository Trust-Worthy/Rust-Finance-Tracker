use std::{io, num::ParseIntError};




pub fn check_user_input(user_choice: &String) -> Result<u32,ParseIntError> {

    let num_choice: Result<u32, ParseIntError> = user_choice.trim().parse();

    return num_choice
}


pub fn get_user_selection() -> String {

    println!("\nOptions are below");
    println!("1. Add a new transaction");
    println!("2. View transactions and summary within a date range");
    println!("3. Exit");
    
    io::stdout().flush().expect("Failed to flush stdout");

    let mut user_choice: String = String::new();


    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read the input!");

    return user_choice

}