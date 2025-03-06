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
    Income(Option<IncomeCategory>, Option<f64>),
    Expense(Option<ExpenseCategory>, Option<f64>)

}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TransactionType::Income(ref category,amount ) => write!(f,"Income: {:?} ${:?}",category,amount),
            TransactionType::Expense(ref category, amount) => write!(f,"Expense: {:?} ${:?}",category,amount)
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub date: Option<NaiveDate>,
    pub amount: Option<f64>,
    pub _type: Option<TransactionType>,
    pub description: Option<String>,

}

impl fmt::Display for Transaction {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date_str = match &self.date {
            Some(date) => date.to_string(),
            None => "N/A".to_string(), // Default for None
        };

        let amount_str = match &self.amount {
            Some(amount) => format!("{:.2}", amount),
            None => "N/A".to_string(), // Default for None
        };

        let type_str = match &self._type {
            Some(TransactionType::Income(_,_)) => "Income".to_string(),
            Some(TransactionType::Expense(_,_)) => "Expense".to_string(),
            None => "N/A".to_string(), // Default for None
        };

        let description_str = match &self.description {
            Some(description) => description.clone(),
            None => "No description".to_string(), // Default for None
        };

        write!(
            f,
            "{:<10} {:>8}   {:<10}   {}", // aligns all the data
            date_str, amount_str, type_str, description_str
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
        .filter(|t| matches!(t._type, Some(TransactionType::Expense(_, _))))
        .map(|t:&Transaction|t.amount.unwrap_or(0.0))
        .sum() // sum the values
    }

    pub fn get_total_income(&self) -> f64 {
        self.transactions
        .iter()
        .filter(|t| matches!(t._type, Some(TransactionType::Income(_, _))))
        .map(|t:&Transaction|t.amount.unwrap_or(0.0))
        .sum() // sum the values
    }

    pub fn show_all_transactions(&self, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>) {
        

        let start_date: NaiveDate = start_date.unwrap_or_else(|| NaiveDate::from_ymd_opt(2000,1,1).expect("Valid default date"));
        let end_date: NaiveDate = end_date.unwrap_or_else(|| NaiveDate::from_ymd_opt(2050,1,1).expect("Vaid default date"));


        println!("Transactions from {} to {}", start_date, end_date);
        println!("{:<15} {:<10} {:<15} {:<50}", "Date", "Amount", "Category", "Description");
        println!("---------------------------------------------------------------");
        for transaction in &self.transactions {
            // Check if the transaction date is within the start and end date
            if let Some(date) = transaction.date {
                if date >= start_date && date <= end_date {
        
                    let category = match &transaction._type {
                        Some(TransactionType::Income(ref category, _)) => match category {
                            Some(IncomeCategory::WORK) => "Work".to_string(),
                            Some(IncomeCategory::FREELANCE) => "Freelance".to_string(),
                            Some(IncomeCategory::GIFT) => "Gift".to_string(),
                            Some(IncomeCategory::OTHER) => "Miscellaneous".to_string(),
                            None => "Unknown".to_string(),
                        },
                        Some(TransactionType::Expense(ref category, _)) => match category {
                            Some(ExpenseCategory::Food) => "Food".to_string(),
                            Some(ExpenseCategory::Transportation) => "Transportation".to_string(),
                            Some(ExpenseCategory::Entertainment) => "Entertainment".to_string(),
                            Some(ExpenseCategory::Rent) => "Rent".to_string(),
                            Some(ExpenseCategory::Giving)=> "Giving".to_string(),
                            Some(ExpenseCategory::Other) => "Other".to_string(),
                            None => "Unknown".to_string(),
                        },
                        None => "Unknown".to_string(), // Handle None case if there's no transaction type
                    };
                    
                    // Format the amount
                    

                    let amount = match &transaction._type {
                        Some(TransactionType::Income(_, Some(amount))) => format!("{:.2}", amount),
                        Some(TransactionType::Expense(_, Some(amount))) => format!("{:.2}", amount),
                        _ => "0.00".to_string(), // Handle the None case or unexpected case
                    };
                    

                    // Print the transaction details
                    println!("{:<15} {:<10} {:<15} {:<50}",
                        date,  // Using `date` directly after unwrap
                        amount,
                        category,
                        transaction.description.as_ref().unwrap_or(&"No description".to_string())); // Handle None for description
                
            }       
        }
    }
        
        

} 


    pub fn show_summary(&self,start_date: Option<NaiveDate>,end_date: Option<NaiveDate>) {

        let income: f64;
        let expense: f64;
    

        income = self.get_total_income();
        expense = self.get_total_expenses();
        let savings:f64 = income - expense;

        self.show_all_transactions(start_date, end_date);
        println!("\n");
        println!("Summary:");
        println!("Total Income: ${income}");
        println!("Total Expenses: ${expense}");
        println!("Net savings: ${savings}");
        
    }

}