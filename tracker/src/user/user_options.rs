use std::{io, num::ParseIntError, process};
use chrono::{format::ParseError, Local, NaiveDate};

use crate::tracker_features::ledger::TransactionType;

pub fn welcome_message() {

    println!("\nWelcome to the finance tracker. You're going to be finacially smart!");
    println!("Please choose something from the options below.");
    
}

pub fn display_menu_options() {
    println!("\nOptions are below");
    println!("1. Add a new transaction");
    println!("2. View transactions and summary within a date range");
    println!("3. Exit");
    println!("Enter your choice(1-3):")
}

pub fn check_user_option_input(user_choice: &String) -> Result<u32,ParseIntError> {

    let num_choice: Result<u32, ParseIntError> = user_choice.trim().parse();

    return num_choice
}

pub fn check_user_date_input(user_choice: &String) -> Result<NaiveDate,ParseError> {
    if user_choice.trim().is_empty() {
        return Ok(Local::now().naive_local().date())
    }

    let date = NaiveDate::parse_from_str(user_choice, "%Y-%m-%d");
    return date
}

pub fn get_user_transaction() -> NaiveDate{

    let mut transaction_date: String = String::new();
    let mut transaction_amount: Result<f64,_>;
    let mut transaction_type: String = String::new();
    let mut transaction_category: String = String::new();
    let mut user_options: UserOptions;

    loop {
        
        println!("Enter HOME at any time to go back to the home menu");
        
        
        loop {

            println!("Enter the date of the transaction (yy-mm-dd), hit 'enter' for today's date:");
            transaction_date.clear();
            io::stdin()
            .read_line(&mut transaction_date)
            .expect("Failed to read the input!");
            
            if transaction_date == "HOME" {
                get_user_menu_selection();
                break;
            }
            match check_user_date_input(&transaction_date) {
                
                Ok(date) => {
                    user_options.date = transaction_date;
                    break;
                }
                Err(_e) => println!("Failed to parse date: {}",_e),
            }
        }

        
        
        loop {
            println!("Enter the amount of the transaction: ");
        
            let mut amount_str = String::new();
            io::stdin()
            .read_line(&mut amount_str)
            .expect("Failed to read the input!");

            transaction_amount = amount_str.parse();

            match transaction_amount {
                Ok(value) => {
                    user_options.amount = value;
                    break;
                }
                Err(e) => println!("Failed to parse {}. Try again!",e),
            }

        }
        
        
        
        println!("Enter the type (Income/Expense)");
        io::stdin()
        .read_line(&mut transaction_type)
        .expect("Failed to read the input!");

    
        match check_user_date_input(&amount_str) {
            Ok(date) => return date,
            Err(_e) => println!("Failed to parse date: {}",_e),
        }
    };
   

}

#[derive(Debug)]
pub struct UserOptions {
    date: NaiveDate,
    amount: f64,
    _type: TransactionType,
    description: String,
}

pub fn get_user_menu_selection() -> u32 {

    let mut user_choice: String = String::new();
    
    loop {
        display_menu_options();

        user_choice.clear(); // Clear previous input before reading again
    
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read the input!");
    
        match check_user_option_input(&user_choice) {
            Ok(selection) if selection == 1 || selection == 2 || selection == 3 => {
    
                return selection;
                
            }
            _ => println!("Please try again. Enter 1, 2, or 3."),

        }
    }

}


pub fn exit_program() {
    process::exit(0);
}
    



    