use chrono::NaiveDate;
use std::fmt;





#[derive(Debug)]
pub enum ExpenseCategory {
    Food,
    Transportation,
    Entertainment,
    Rent,
    Giving,
    Other,
}

impl fmt::Display for ExpenseCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let category_str = match *self {
            ExpenseCategory::Food => "Food",
            ExpenseCategory::Transportation => "Transportation",
            ExpenseCategory::Entertainment => "Entertainment",
            ExpenseCategory::Rent => "Rent",
            ExpenseCategory::Giving => "Giving",
            ExpenseCategory::Other => "Miscellaneous",
        };

        return write!(f,"{}",category_str)
    }
}

#[derive(Debug)]
struct Transaction {
    date: NaiveDate,
    amount: f64,
    category: ExpenseCategory,
    description: String,

}

impl fmt::Display for Transaction {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(
            f,
            "{:<10} {:>8.2}   {:<10}   {}", // aligns all the data
            self.date, self.amount,self.category, self.description
        )
    }
}

#[derive(Debug)]
struct Ledger {

    transactions: Vec<Transaction>,
}

impl Ledger {

    fn new() -> Self {
        Ledger {
            transactions: Vec::new(),
        }

    }

    fn add_transaction(&mut self, transaction: Transaction) {

        self.transactions.push(transaction);
    }

    fn get_total_spent(&self) -> f64 {
        self.transactions.iter().map(|t:&Transaction|t.amount).sum()
    }

    fn show_all_transactions(&self, start_date: NaiveDate, end_date: NaiveDate) {

    } 

    fn get_transactions_by_category(&self, category: ExpenseCategory) -> {
        
    }

}