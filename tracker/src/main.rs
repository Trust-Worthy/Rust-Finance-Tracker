//! Finance Tracker in Rust!
//! 
//! Learning by doing is the best type of learning in my book!

mod user;
use chrono::NaiveDate;
use user::user_options::{welcome_message,get_user_menu_selection,exit_program,create_user_transaction,get_user_summary_range};

mod tracker_features;
use tracker_features::ledger::{Ledger, Transaction};


fn run_program(user_ledger: &mut Ledger) {

    loop {

        let user_selection: u32 = get_user_menu_selection();
    
        
    
        match user_selection {
            3 => {
                println!("Exiting Program...");
                exit_program();
            }    
            2 => {
                let (start_date,end_date): (NaiveDate,NaiveDate) = get_user_summary_range();
                user_ledger.show_all_transactions(Some(start_date), Some(end_date));
                user_ledger.show_summary(Some(start_date), Some(end_date));
            },
            1 => {
                let user_transaction:Transaction = create_user_transaction();
                user_ledger.add_transaction(user_transaction);
                println!("Entry Added successfully");
            }
            _ => println!("Invalid Selection. Please try again."),
        }
    }
}


fn main() {
    welcome_message();
    let mut user_ledger:Ledger = Ledger::new();

    run_program(&mut user_ledger);
    


   
}
