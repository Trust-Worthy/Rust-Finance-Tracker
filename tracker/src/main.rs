//! Finance Tracker in Rust!
//! 
//! Learning by doing is the best type of learning in my book!

mod user;
use user::user_options::{welcome_message,get_user_menu_selection,exit_program,create_user_transaction,get_user_summary_range};

mod tracker_features;
use tracker_features::ledger::{Ledger, Transaction};





fn main() {
    welcome_message();
    let mut user_ledger:Ledger = Ledger::new();


    loop {

        let user_selection: u32 = get_user_menu_selection();
    
        
    
        match user_selection {
            3 => exit_program(),    
            2 => {
                let (start_dateend_date): (String,String) = get_user_summary_range();
            },
            1 => {
                let user_transaction:Transaction = create_user_transaction();
                user_ledger.add_transaction(user_transaction);
                println!("Entry Added successfully");
            }
            _ => println!("Invalid ")
        }
    }


   
}
