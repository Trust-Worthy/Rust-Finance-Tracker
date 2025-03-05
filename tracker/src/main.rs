//! Finance Tracker in Rust!
//! 
//! Learning by doing is the best type of learning in my book!

mod user;
use user::user_options::{welcome_message,get_user_selection};

mod tracker_features;





fn main() {
    welcome_message();
    let user_selection: u32 = get_user_selection();
    
   
}
