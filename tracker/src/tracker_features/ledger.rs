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
        let category_str: &str = match *self {
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
pub enum IncomeCategory {
    WORK,
    FREELANCE, 
    GIFT,
    OTHER,
}

impl fmt::Display for IncomeCategory {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        let category_str: &str = match *self {
            IncomeCategory::WORK => "Work",
            IncomeCategory::FREELANCE => "Freelance",
            IncomeCategory::GIFT => "Gift",
            IncomeCategory::OTHER => "Miscellaneous",
        };

        return write!(f,"{}",category_str)
    }
}

#[derive(Debug)]
pub enum TransactionType {
    Income(IncomeCategory, f64),
    Expense(ExpenseCategory, f64)

}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TransactionType::Income(ref category,amount ) => write!(f,"Income: {} ${}",category,amount),
            TransactionType::Expense(ref category, amount) => write!(f,"Expense: {} ${}",category,amount)
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    date: NaiveDate,
    amount: f64,
    _type: TransactionType,
    description: String,

}

impl fmt::Display for Transaction {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(
            f,
            "{:<10} {:>8.2}   {:<10}   {}", // aligns all the data
            self.date, self.amount,self._type, self.description
        )
    }
}

#[derive(Debug)]
pub struct Ledger {

    transactions: Vec<Transaction>,
}

impl Ledger {

    pub fn new() -> Self {
        Ledger {
            transactions: Vec::new(),
        }

    }

    pub fn add_transaction(&mut self, transaction: Transaction) {

        self.transactions.push(transaction);
    }

    pub fn get_total_expenses(&self) -> f64 {
        self.transactions
        .iter()
        .filter(|t| matches!(t._type, TransactionType::Expense(_, _)))
        .map(|t:&Transaction|t.amount)
        .sum() // sum the values
    }

    pub fn get_total_income(&self) -> f64 {
        self.transactions
        .iter()
        .filter(|t| matches!(t._type, TransactionType::Income(_, _)))
        .map(|t:&Transaction|t.amount)
        .sum() // sum the values
    }

    pub fn show_all_transactions(&self, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>) {
        

        let start_date: NaiveDate = start_date.unwrap_or_else(|| NaiveDate::from_ymd_opt(2000,1,1).expect("Valid default date"));
        let end_date: NaiveDate = end_date.unwrap_or_else(|| NaiveDate::from_ymd_opt(2050,1,1).expect("Vaid default date"));


        println!("Transactions from {} to {}", start_date, end_date);
        println!("{:<15} {:<10} {:<15} {:<50}", "Date", "Amount", "Category", "Description");
        println!("---------------------------------------------------------------");
        for transaction in &self.transactions {
            if transaction.date >= start_date && transaction.date <= end_date {
                // Get the category as a string representation
                let category = match &transaction._type {
                    TransactionType::Income(ref category,_) => category.to_string(),
                    TransactionType::Expense(ref category,_) => category.to_string(),
                };
                

                let amount = match &transaction._type {
                    TransactionType::Income(_,amount) => format!("{:2}",amount),
                    TransactionType::Expense(_,amount) => format!("{:2}",amount),
                };

                println!("{:<15} {:<10} {:<15} {:<50}", 
                transaction.date, 
                amount, 
                category, 
                transaction.description);

            }
        }

    } 


    pub fn show_summary(&self,start_date: Option<NaiveDate>,end_date: Option<NaiveDate>) {

        let income: f64;
        let expense: f64;
    

        income = self.get_total_income();
        expense = self.get_total_income();
        let savings:f64 = income - expense;

        self.show_all_transactions(start_date, end_date);
        println!("\n");
        println!("Summary:");
        println!("Total Income: ${income}");
        println!("Total Expenses: ${expense}");
        println!("Net savings: ${savings}");
        
    }

}