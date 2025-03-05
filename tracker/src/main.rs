//! Finance Tracker in Rust!
//! 
//! Learning by doing is the best type of learning in my book!

mod user;
use user::user_options::{welcome_message,get_user_menu_selection,exit_program,get_user_transaction};

mod tracker_features;
use tracker_features::ledger::Ledger;





fn main() {
    welcome_message();
    let mut user_ledger:Ledger = Ledger::new();


    loop {

        let user_selection: u32 = get_user_menu_selection();
    
        
    
        match user_selection {
            3 => exit_program(),    
            2 => get_user_transaction(),
            1 => 
        }
    }


   
}
