use chrono::NaiveDate;






#[derive(Debug)]
pub enum ExpenseCategory {
    Food,
    Transportation,
    Entertainment,
    Rent,
    Giving,
    Other,
}

#[derive(Debug)]
struct Transaction {
    date: NaiveDate,
    amount: f64,
    category: ExpenseCategory

}

#[derive(Debug)]
struct Ledger {

    transactions: Vec<Transaction>,
}

impl Ledger {

    fn new() -> Self {
        
    }

}