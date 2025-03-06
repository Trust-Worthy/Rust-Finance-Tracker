use std::{io, num::ParseIntError, process};
use chrono::{format::ParseError, Local, NaiveDate};

use crate::tracker_features::ledger::ExpenseCategory;
use crate::tracker_features::ledger:: IncomeCategory;
use crate::tracker_features::ledger::Transaction;
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

    let date = NaiveDate::parse_from_str(user_choice.trim(), "%Y-%m-%d");
    return date
}

pub fn create_user_transaction() -> Transaction{

    let mut transaction_date: String = String::new();
    let mut transaction_amount: Result<f64,_>;
    let mut transaction_type: String = String::new();
    let mut transaction_category: String = String::new();
    let mut transaction_description: String = String::new();
    let mut user_options: Transaction = Transaction {
        date: None,
        amount: None,
        _type: None,
        description:None
    };  

    'get_all_details: loop {
        
        println!("Enter HOME at any time to go back to the home menu");
        
        
        'date:loop {

            println!("Enter the date of the transaction (yyyy-mm-dd), hit 'enter' for today's date:");
            transaction_date.clear();
            io::stdin()
            .read_line(&mut transaction_date)
            .expect("Failed to read the input!");
            
            if transaction_date == "HOME" {
                get_user_menu_selection();
                break 'get_all_details;
            }
            match check_user_date_input(&transaction_date) {
                Ok(date) => {
                    user_options.date = Some(date);
                    break 'date;
                }
                Err(_e) => println!("Failed to parse date: {}",_e),
            }
        }

        
        
        'amount:loop {
            println!("Enter the amount of the transaction: ");
        
            let mut amount_str = String::new();
            io::stdin()
            .read_line(&mut amount_str)
            .expect("Failed to read the input!");

            if amount_str == "HOME" {
                get_user_menu_selection();
                break 'get_all_details;
            }

            transaction_amount = amount_str.parse();

            match transaction_amount {
                Ok(value) => {
                    user_options.amount = Some(value);
                    break 'amount;
                }
                Err(e) => println!("Failed to parse {}. Try again!",e),
            }

        }
        
        
        'transaction_type:loop {

            println!("Enter the type (Income/Expense)");
            io::stdin()
            .read_line(&mut transaction_type)
            .expect("Failed to read the input!");

            if transaction_type == "HOME" {
                get_user_menu_selection();
                break 'get_all_details;
            }

            println!("Enter the category:");
            println!("Income Categories: Work, Freelance, Gift");
            println!("Expense Categories: Food,Transportt, Entertainment, Rent, Giving");
            
            io::stdin()
            .read_line(&mut transaction_category)
            .expect("Failed to read the input!");
            
            if transaction_category == "HOME" {
                get_user_menu_selection();
                break 'get_all_details;
            } 
        
            match transaction_type.as_str() {
                "Income" => {
                    let category: IncomeCategory;
                    match transaction_category.as_str() {
                        "Work" => category = IncomeCategory::WORK,
                        "Freelance" => category = IncomeCategory::FREELANCE,
                        "Gift" => category = IncomeCategory::GIFT,
                        _ => category = IncomeCategory::OTHER

                    }
                    user_options._type = Some(TransactionType::Income(Some(category), user_options.amount));
                    break 'transaction_type;
                }
                "Expense" => {
                    let category: ExpenseCategory;
                    match transaction_category.as_str() {
                        "Food" => category = ExpenseCategory::Food,
                        "Transport" => category = ExpenseCategory::Transportation,
                        "Entertainment" => category = ExpenseCategory::Entertainment,
                        "Rent" => category = ExpenseCategory::Rent,
                        "Giving" => category = ExpenseCategory::Giving,
                        _ => category = ExpenseCategory::Other
                    }
                    user_options._type = Some(TransactionType::Expense(Some(category), user_options.amount));
                    break 'transaction_type;
                }
                _ => println!("Please enter in a correct type")

            };
        }

        println!("Enter a description:");
        io::stdin()
            .read_line(&mut transaction_description)
            .expect("Failed to read the input!");
            
            if transaction_description == "HOME" {
                get_user_menu_selection();
                break 'get_all_details;
            }

            user_options.description = Some(transaction_description);
            break 'get_all_details;
    };

    
    return user_options
   

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

pub fn get_user_summary_range() -> (NaiveDate,NaiveDate){


    let mut summary_start_date: String = String::new();
    let mut summary_end_date: String = String::new();
    let start_date: NaiveDate;
    let end_date: NaiveDate;

    println!("Hit Enter for next two options to get all transactions");

    'start_date: loop {
        println!("Enter the start date (yyyy-mm-dd): ");

        summary_start_date.clear();
        io::stdin()
            .read_line(&mut summary_start_date)
            .expect("Failed to read the input!");

        start_date = match summary_start_date.trim() {
            "" => {
                NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() // Default if empty
            }
            _ => match NaiveDate::parse_from_str(summary_start_date.as_str(), "%Y-%m-%d") {
                Ok(date) => {
                    date // Use valid parsed date
                }
                Err(e) => {
                    println!("Failed to parse date: {}", e);
                    NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() // Default on error
                }
            }
        };

        // Break the loop once we have a valid date
        println!("Using start date: {}", start_date);
        break 'start_date;
    }
    
    'end_date: loop {
        println!("Enter the end date (yyyy-mm-dd): ");
        summary_end_date.clear();
        io::stdin()
            .read_line(&mut summary_end_date)
            .expect("Failed to read the input!");

        end_date = match summary_end_date.trim() {
            "" => {
                NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() // Default if empty
            }
            _ => match NaiveDate::parse_from_str(summary_end_date.as_str(), "%Y-%m-%d") {
                Ok(date) => {
                    date // Use valid parsed date
                }
                Err(e) => {
                    println!("Failed to parse date: {}", e);
                    NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() // Default on error
                }
            }
        };

        // Break the loop once we have a valid date
        println!("Using end date: {}", end_date);
        break 'end_date;
    }

    return (start_date,end_date)

}


pub fn exit_program() {
    process::exit(0);
}
    



    