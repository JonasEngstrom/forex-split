use clap::Parser;
use std::io;
use std::io::Write;
use std::process::exit;
use std::collections::HashMap;

/// Flush to stdout. If it is not possible, print an error message and exit the program.
macro_rules! flush {
    () => {
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Unable to write terminal output.");
                exit(1);
            },
        }
    };
}

/// Get input from terminal and store it in a variable.
macro_rules! input {
    ($input:ident) => {
        let mut $input = String::new();
        match io::stdin().read_line(&mut $input) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Unable to read terminal input.");
                exit(1);
            },
        }
    };
}

/// Split a receipt in one currency into categories for bookkeeping in another currency.
#[derive(Parser)]
pub struct Args {
    /// Total in the currency from your bank statement, i.e. probably the currency domestic to your home country.
    domestic_total: Option<f64>,
    /// Total in the currency on the receipt from the purchase, i.e. the, to you, foreign currency from the country you are traveling in.
    foreign_total: Option<f64>,
}

impl Args {
    /// Parse command line arguments.
    pub fn new() -> Self {
        Self::parse()
    }

    /// Get a total from command line arguments, if available, otherwise ask the user to enter a total.
    fn get_total(desired_total: Option<f64>, prompt: &str) -> f64 {
        match desired_total {
            Some(total) => total,
            None => {
                print!("{}", prompt);
                flush!();
                input_float()
            },
        }
    }

    /// Return the domestic total from the command line arguments, if one exists. Otherwise ask the user to provide a domestic total.
    fn domestic_total(&self) -> f64 {
        Self::get_total(self.domestic_total, "Please enter domestic total: ")
    }

    /// Return the foreign total from the command line arguments, if one exists. Otherwise ask the user to provide a foreign total.
    fn foreign_total(&self) -> f64 {
        Self::get_total(self.foreign_total, "Please enter foreign total: ")
    }
}

/// Get terminal input and parse it to an f64. If no string that can be parsed to an f64 is provided, ask the user to provide a new string, until one that can be parsed to and f64 is provided. Prints an error message and exits the program if unable to write get input from the terminal.
fn input_float() -> f64 {
    let parsed_input = loop {
        input!(input);
        match input
            .trim()
            .parse::<f64>() {
                Ok(parsed_input) => break parsed_input,
                Err(_) => {
                    print!("Please try again. Enter a valid number: ");
                    flush!();
                    continue;
                },
            }
    };
    parsed_input
}

/// Store the conversion factor between the domestic and the foreign currency, as well as the category subtotals.
pub struct CategoryList {
    /// The conversion factor between the domestic and the foreign currency.
    conversion_factor: f64,
    /// The part of the foreign currency amount that has not yet been assigned to a category.
    remaining_foreign_total: f64,
    /// A list of categories and corresponding subtotals.
    categories: HashMap<String, f64>,
}

impl CategoryList {
    /// Create a new CategoryList, by taking an Args as the only argument.
    pub fn new(args: Args) -> Self {
        let domestic_total = args.domestic_total();
        let foreign_total = args.foreign_total();
        Self {
            conversion_factor: domestic_total / &foreign_total,
            remaining_foreign_total: foreign_total,
            categories: HashMap::new(),
        }
    }

    /// Return the assigned total.
    pub fn category_grand_total(&self) -> f64 {
        let mut grand_total: f64 = 0f64;

        for (_, &value) in &self.categories {
            grand_total += value;
        }

        grand_total
    }

    /// Add a value to a category or, if the category does not yet exist, create the category and assign the value.
    pub fn assign_to_category(&mut self, category_name: String, value_to_assign: f64) -> () {
        match &self.categories.get(&category_name) {
            &Some(&value) => &self.categories.insert(category_name, &value + value_to_assign),
            None => &self.categories.insert(category_name, value_to_assign),
        };
    }

    /// Get user input of categories.
    pub fn input_loop(&self) -> () {
        while self.remaining_foreign_total > 0f64 {
            print!("Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: {}): ", &self.remaining_foreign_total);
            flush!();
            input!(input);
        }
    }
}