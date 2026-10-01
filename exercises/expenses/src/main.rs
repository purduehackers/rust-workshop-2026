// Expense tracker.
//
//   cargo run -- add coffee 4 food     record an expense
//   cargo run -- list                  print every expense and the total
//   cargo run -- biggest               print the biggest expense
//   cargo run -- remove coffee         delete an expense by name
//   cargo run -- --help                list every command
//
// Expenses are kept in expenses.txt between runs. Parts 1 to 5 are methods
// whose body is `todo!()`: replace each one, top to bottom. Part 6 adds a
// command. `main` shows how everything is called. When you are done,
// `sh demo.sh` prints exactly what is in expected.txt.

// Parts you have not written yet leave some code unused.
#![allow(dead_code, unused_variables)]

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

// 1. Print one line per expense, then "Total: $<total>".
//    Hint: `print_all` does the first half, `self.total()` the second.
impl Describe for Expenses {
    fn describe(&self) {
        todo!()
    }
}

// 2. Print "<name>: $<amount> (<category>)" with the category in lowercase,
//    for example "coffee: $4 (food)".
//    Hint: `match self.category { Category::Food => "food", ... }`.
impl Describe for Expense {
    fn describe(&self) {
        todo!()
    }
}

impl Expenses {
    // Given, and the shape every method below follows. `&mut self` because
    // it changes the list; the others only read it and take `&self`.
    fn add(&mut self, expense: Expense) {
        self.list.push(expense);
    }

    // 3. The sum of every amount.
    fn total(&self) -> u32 {
        todo!()
    }

    // 4. The expense with the largest amount. You may assume there is one.
    //    Hint: start with `let mut biggest: &Expense = &self.list[0];`.
    fn biggest(&self) -> &Expense {
        todo!()
    }

    // 5. Deletes the first expense called `name` and prints
    //    "Removed <name>: $<amount>". If there is none, prints
    //    "No expense named <name>".
    //    Hint: find the index with a for loop first. `self.list.remove(index)`
    //    takes the expense out of the list and gives it back to you.
    fn remove(&mut self, name: &str) {
        todo!()
    }

    // 6. Add a `spent` command: `cargo run -- spent food` prints
    //    "Spent on Food: $<sum>", the sum of the amounts in that category.
    //    You need a variant in `Cmd`, a match arm in `main`, and a method here.
    //    Hint: `{category:?}` prints a Category as "Food".
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
    }

    expenses.save();
}

// Given: saving and loading expenses.txt, one "<name> <amount> <category>"
// per line. Nothing to change below this point.
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
