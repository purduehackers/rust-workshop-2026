// Expense tracker, finished. Run it with
//
//   cargo run --bin expenses-solution -- add coffee 4 food
//   cargo run --bin expenses-solution -- list
//   cargo run --bin expenses-solution -- biggest
//   cargo run --bin expenses-solution -- remove coffee
//   cargo run --bin expenses-solution -- spent food
//   cargo run --bin expenses-solution -- --help
//
// Expenses are kept in expenses.txt between runs. `sh demo.sh expenses-solution`
// prints exactly what is in expected.txt.

use std::fs;

use clap::{Parser, ValueEnum};

const FILE: &str = "expenses.txt";

// A Category enum. `ValueEnum` lets Clap read it from the command line,
// typed in lowercase: `food`, `rent` or `travel`.
#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
enum Category {
    Food,
    Rent,
    Travel,
}

// One expense: what it was, how much it cost, and its category.
struct Expense {
    name: String,
    amount: u32,
    category: Category,
}

// Every expense, in the order they were added.
struct Expenses {
    list: Vec<Expense>,
}

/// Expense tracker
// `Parser` on an enum makes each variant a command and each field one of
// its arguments: `cargo run -- add coffee 4 food` becomes
// `Cmd::Add { name: "coffee", amount: 4, category: Category::Food }`.
#[derive(Parser)]
enum Cmd {
    /// Record an expense
    Add { name: String, amount: u32, category: Category },
    /// Print every expense and the total
    List,
    /// Print the biggest expense
    Biggest,
    /// Delete an expense by name
    Remove { name: String },
    /// Print how much was spent in one category
    Spent { category: Category },
}

// A Describe trait: shared behavior for anything that can print itself.
trait Describe {
    fn describe(&self);
}

// Given. Prints every item. Works for any type that implements Describe.
fn print_all<T: Describe>(items: &[T]) {
    for item in items {
        item.describe();
    }
}

// 1. Every expense, then the total.
impl Describe for Expenses {
    fn describe(&self) {
        print_all(&self.list);
        println!("Total: ${}", self.total());
    }
}

// 2. One line per expense, with the category in lowercase.
impl Describe for Expense {
    fn describe(&self) {
        let category: &str = match self.category {
            Category::Food => "food",
            Category::Rent => "rent",
            Category::Travel => "travel",
        };
        println!("{}: ${} ({category})", self.name, self.amount);
    }
}

impl Expenses {
    // Given. `&mut self` because it changes the list.
    fn add(&mut self, expense: Expense) {
        self.list.push(expense);
    }

    // 3. The sum of every amount.
    fn total(&self) -> u32 {
        let mut sum: u32 = 0;
        for expense in &self.list {
            sum += expense.amount;
        }
        sum
    }

    // 4. The expense with the largest amount. You may assume there is one.
    fn biggest(&self) -> &Expense {
        let mut biggest: &Expense = &self.list[0];
        for expense in &self.list {
            if expense.amount > biggest.amount {
                biggest = expense;
            }
        }
        biggest
    }

    // 5. Deletes the first expense called `name`, or says there is none.
    fn remove(&mut self, name: &str) {
        // Option is an enum with two variants: Some(index) once we find a
        // match, None if we never do.
        let mut found: Option<usize> = None;
        for (index, expense) in self.list.iter().enumerate() {
            if expense.name == name {
                found = Some(index);
                break;
            }
        }
        match found {
            Some(index) => {
                let expense: Expense = self.list.remove(index);
                println!("Removed {}: ${}", expense.name, expense.amount);
            }
            None => println!("No expense named {name}"),
        }
    }

    // 6. The sum of the amounts in one category.
    fn spent(&self, category: Category) -> u32 {
        let mut sum: u32 = 0;
        for expense in &self.list {
            if expense.category == category {
                sum += expense.amount;
            }
        }
        sum
    }
}

fn main() {
    let mut expenses: Expenses = Expenses::load();

    match Cmd::parse() {
        Cmd::Add { name, amount, category } => {
            expenses.add(Expense { name, amount, category });
        }
        Cmd::List => expenses.describe(),
        Cmd::Biggest => {
            if expenses.list.is_empty() {
                println!("No expenses yet");
            } else {
                expenses.biggest().describe();
            }
        }
        Cmd::Remove { name } => expenses.remove(&name),
        Cmd::Spent { category } => {
            let sum: u32 = expenses.spent(category);
            println!("Spent on {category:?}: ${sum}");
        }
    }

    expenses.save();
}

// Given: saving and loading expenses.txt, one "<name> <amount> <category>"
// per line.
impl Expenses {
    fn load() -> Self {
        // An empty String when the file does not exist yet.
        let text: String = fs::read_to_string(FILE).unwrap_or_default();
        let mut list: Vec<Expense> = Vec::new();
        for line in text.lines() {
            // `rsplit_once` cuts the line at its last space, so names may
            // contain spaces. `expect` stops the program if a line is broken.
            let (rest, category) = line.rsplit_once(' ').expect("bad line in expenses.txt");
            let (name, amount) = rest.rsplit_once(' ').expect("bad line in expenses.txt");
            let category: Category = match category {
                "Food" => Category::Food,
                "Rent" => Category::Rent,
                _ => Category::Travel,
            };
            let amount: u32 = amount.parse().expect("bad amount in expenses.txt");
            list.push(Expense { name: name.to_string(), amount, category });
        }
        Expenses { list }
    }

    fn save(&self) {
        let mut text: String = String::new();
        for expense in &self.list {
            text.push_str(&format!("{} {} {:?}\n", expense.name, expense.amount, expense.category));
        }
        fs::write(FILE, text).expect("could not write expenses.txt");
    }
}
