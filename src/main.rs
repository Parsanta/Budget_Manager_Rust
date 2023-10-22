use std::{collections::HashMap, io};

struct BudgetManager {
    budgets: HashMap<String, Budget>,
}
#[derive(Debug)]
struct Budget {
    name: String,
    amount: i64,
    transactions: Vec<Transaction>,
}
#[derive(Debug)]
struct Transaction {
    desc: String,
    amount: i64,
}

impl BudgetManager {
    fn new() -> Self {
        BudgetManager {
            budgets: HashMap::new(),
        }
    }

    fn add_budget(&mut self, name: String, amount: i64) {
        let budget = Budget {
            name: name.clone(),
            amount,
            transactions: Vec::new(),
        };
        self.budgets.insert(name, budget);
    }

    fn remove_budget(&mut self, name: &str) {
        self.budgets.remove(name);
    }

    fn edit_budget(&mut self, name: &str, amount: i64) {
        if let Some(budget) = self.budgets.get_mut(name) {
            budget.amount = amount;
        }
    }

    fn add_transaction(&mut self, name: &str, desc: String, amount: i64) {
        if let Some(budget) = self.budgets.get_mut(name) {
            let transaction = Transaction {  amount, desc };
            budget.transactions.push(transaction);
        }
    }

    fn show_budget(&mut self, name: &str) {
        if let Some(budget) = self.budgets.get(name) {
            println!("{:#?}", budget);
        };
    }
}

fn main() {
    let mut input = String::new();
    let mut name = String::new();
    let mut amount_str = String::new();
    let mut budget_manager = BudgetManager::new();
    let mut desc = String::new();

    loop {
        println!("Welcome to Budget Manager:");
        println!("1) Add budget");
        println!("2) Delete budget");
        println!("3) Edit budget");
        println!("4) Show budget");
        println!("5) Add transaction");
        println!("6) Quit");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid");

        match choice {
            1 => {
                println!("Write a name for a budget:");
                let mut name=String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Invalid");
                name = name.trim().to_string();

                println!("Enter the amount of budget:");
                let mut amount_str =String::new();
                io::stdin()
                    .read_line(&mut amount_str)
                    .expect("Invalid");

                let amount: i64 = match amount_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number.");
                        return;
                    }
                };
                budget_manager.add_budget(name, amount);
            }

            2 => {
                println!("Enter the name of the budget to be deleted");
                let mut name=String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Invalid");
                name = name.trim().to_string();
                budget_manager.remove_budget(&name);
            }

            3 => {
                println!("enter the budget name :");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Invalid");
                name = name.trim().to_string();
                println!("Enter the new budget amount:");
                let mut amount_str =String::new();
                io::stdin()
                    .read_line(&mut amount_str)
                    .expect("Invalid");

                let amount= match amount_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount number.");
                        return;
                    }
                };
                budget_manager.edit_budget(&name, amount)
            }
            4 => {
                println!("Enter budget name to show :");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Invalid");
                name = name.trim().to_string();
                budget_manager.show_budget(&name)
            }
            5 => {
                println!("Enter name of a budget:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Invalid");
                name = name.trim().to_string();
                println!("Enter transaction desc:");
                let mut desc = String::new();
                io::stdin()
                    .read_line(&mut desc)
                    .expect("Invalid desc");
                desc = desc.trim().to_string();
                println!("Enter transaction amount:");
                io::stdin()
                    .read_line(&mut amount_str)
                    .expect("Invalid amount");

                let amount: i64 = match amount_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount");
                        return;
                    }
                };
                budget_manager.add_transaction(&name, desc, amount);
            }
            6 => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
