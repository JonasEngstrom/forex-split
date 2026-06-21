//! # Forex Split
//! 
//! This crate was made to split a receipt in one currency into categories for bookkeeping in another currency, in order to facilitate bookkeeping in [YNAB](https://www.ynab.com/) while travelling in other countries.
//! 
//! ## Use Case
//! 
//! Say you go out to a restaurant in Germany and pay with your Swedish card. Your receipt might be itemized as follows.
//! 
//! ```bash
//!                           EUR
//! 1 x Bier 0,4l            6,90
//! 1 x Currywurst          17,50
//! 
//! Summe                   24,40
//! Trinkgeld                2,44
//! 
//! Kreditkarte             26,84
//! ```
//! 
//! Your bank statement, on the other hand is in Swedish crowns.
//! 
//! ```bash
//!                           SEK
//! Tysk restaurang       -299,28
//! ```
//! 
//! You want to categorize the beer, the sausage, and the tip into alcohol, eating out, and tip categories respectively, in your Swedish crown YNAB budget. This is what this crate is for.
//! 
//! ## Usage
//! 
//! ### Calling the Program
//! 
//! Using the numbers from our [use case](#use-case) above we can call forex-split with one or two positional arguments. The first being the number from our bank statement and the second being the total from the receipt:
//! 
//! ```bash
//! forex-split 299.28 26.84
//! ```
//! 
//! The program will ask for any missing arguments:
//! 
//! ```bash
//! forex-split 299.28
//! Please enter foreign total: 26.84
//! ```
//! 
//! ```bash
//! forex-split
//! Please enter domestic total: 299.28
//! Please enter foreign total: 26.84
//! ```
//! 
//! The terms domestic and foregin are taken to mean domestic to your home country (SEK in our example) and foreign as in the currency of the, to you, foreign country in which you are travelling (EUR in our example).
//! 
//! ### Running the Program
//! 
//! After entering the totals in both domestic and foreign currencies, you will be asked to assign sums in the foreign currency to categories until either the total equals or exceeds the total stated on the receipt or you assign the remainder of the total from the receipt to a category, called *Other*, by not explicitly typing a number in. Naming categories is optional, but if a category is named several entries can be made into the same category, giving the total value of the category at the end.
//! 
//! > **NOTE!** Note that output is rounded to two decimals.
//! 
//! #### Manually Assigning All Categories
//! 
//! ```bash
//! forex-split 299.28 26.84
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): Alcohol 6.9
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): Food 17.5
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44): Tip 2.44
//! Category    Foreign subtotal    Domestic subtotal
//! Alcohol                 6.90                76.94
//! Food                   17.50               195.13
//! Tip                     2.44                27.21
//! ```
//! 
//! #### Using the Other Category
//! 
//! ```bash
//! forex-split 299.28 26.84
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): Alcohol 6.9
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): Food 17.5
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44):
//! Category    Foreign subtotal    Domestic subtotal
//! Alcohol                 6.90                76.94
//! Food                   17.50               195.13
//! Other                   2.44                27.21
//! ```
//! 
//! #### Using Unnamed Categories
//! 
//! ```bash
//! forex-split 299.28 26.84
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): 6.9
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): 17.5
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44): 2.44
//! Category    Foreign subtotal    Domestic subtotal
//! Unnamed Category 1      6.90                76.94
//! Unnamed Category 2     17.50               195.13
//! Unnamed Category 3      2.44                27.21
//! ```
//! 
//! #### Using Unnamed Categories and the Other Category
//! 
//! ```bash
//! forex-split 299.28 26.84
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): 6.9
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): 17.5
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44):
//! Unnamed Category 1      6.90                76.94
//! Unnamed Category 2     17.50               195.13
//! Other                   2.44                27.21
//! ```
//! 
//! #### Adding Several Items to the Same Category
//! 
//! When you have several items on a receipt belonging to the same category is when bookkeeping while traveling can become a bit complicated. This is where forex-split can really help. Say you had two smaller dishes instead of a larger one. You can just enter them as belonging to the same category and forex-split will add them up for you.
//! 
//! ```bash
//! forex-split 299.28 26.84
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): Alcohol 6.9
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): Food 9.94
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 10.00): Food 7.56
//! Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44): Tip 2.44
//! Category    Foreign subtotal    Domestic subtotal
//! Alcohol                 6.90                76.94
//! Food                   17.50               195.13
//! Tip                     2.44                27.21
//! ```
use clap::Parser;
use std::io;
use std::io::Write;
use std::process::exit;
use std::collections::HashMap;
use std::fmt::Debug;

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
struct Args {
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
#[derive(Debug)]
struct CategoryList {
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

    /// Add a value to a category or, if the category does not yet exist, create the category and assign the value.
    fn assign_to_category(&mut self, category_name: String, value_to_assign: f64) -> () {
        match &self.categories.get(&category_name) {
            &Some(&value) => &self.categories.insert(category_name, &value + value_to_assign),
            None => &self.categories.insert(category_name, value_to_assign),
        };
    }

    /// Get user input of categories.
    pub fn input_loop(&mut self) -> () {
        type CounterType = u64;
        let mut category_counter: CounterType = 0;

        loop {
            print!("Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: {:.2}): ", &self.remaining_foreign_total);
            flush!();
            input!(input);

            // Check whether input is parsable, otherwise print an error message and try again.
            match input
                .trim()
                .parse::<String>() {
                    Ok(parsed_input) => {
                        let mut split_input = parsed_input
                            .split_whitespace()
                            .collect::<Vec<_>>();
                        
                        // Check whether input was provided, otherwise assign remaining money to a category named Other and exit loop.
                        let subtotal = match split_input.last() {
                            Some(subtotal) => {

                                // Check whether the last element on the line is an f64, otherwise print an error message and try again.
                                match subtotal.parse::<f64>() {
                                    Ok(subtotal) => subtotal,
                                    Err(_) => {
                                        eprintln!("Please enter the subtotal as the last element on the line, preceeded by a space. Try again.");
                                        continue;
                                    }
                                }
                            },
                            None => {
                                self.assign_to_category("Other".to_string(), self.remaining_foreign_total);
                                break;
                            }
                        };

                        // Check whether a high enough amount remains to subtract the subtotal.
                        if self.remaining_foreign_total - subtotal < 0f64 {
                            eprintln!("The entered subtotal exceeds the remaining total. Try again or just press enter to assign the entire remaining total to a category named Other.");
                            continue;
                        }

                        // Prepare a category name.
                        split_input.remove(split_input.len() - 1);
                        let mut category_name = split_input.join(" ");

                        // If no name for a category was provided, make a new unnamed, numbered category.
                        if category_name == "" {
                            // Check so that integer overflow does not occur.
                            category_counter =  match category_counter.checked_add(1) {
                                Some(new_counter_value) => new_counter_value,
                                None => {
                                    println!("A maximum of {} unnamed categories are supported. Please name category explicitly. Try again.", CounterType::MAX);
                                    continue;
                                }
                            };

                            // Make new unnamed category name.
                            category_name = format!("Unnamed category {}", category_counter);
                        }

                        // Assign subtotal to category.
                        self.assign_to_category(category_name, subtotal);

                        // Subtract the subtotal from the remaining amout.
                        self.remaining_foreign_total -= subtotal;
                        
                        // Exit loop if all money has been assigned.
                        if self.remaining_foreign_total == 0f64 {
                            break;
                        }
                    },
                    Err(_) => {
                        eprintln!("Unable to parse terminal input. Try again.");
                        continue;
                    },
                }
        }
    }

    /// Returns a sorted list of category names, with named categories first, unnamed categories in numerical order second, and the Other category last.
    fn ordered_category_list(&self) -> Vec<&String> {
        let mut named_categories = Vec::new();
        let mut unnamed_categories = Vec::new();
        let mut other_category = Vec::new();

        for category in self.categories.keys().collect::<Vec<_>>() {
            if *category == "Other" {
                other_category.push(category);
                continue;
            } else if category.starts_with("Unnamed category") {
                unnamed_categories.push(category);
                continue;
            } else {
                named_categories.push(category);
                continue;
            }
        }
        
        named_categories.sort();
        unnamed_categories.sort();
        
        let mut ordered_categories = named_categories;
        ordered_categories.append(&mut unnamed_categories);
        ordered_categories.append(&mut other_category);
        
        ordered_categories
    }

    /// Prints a table of categories with foreign amounts converted to the domestic currency.
    pub fn print_split_table(&self) -> () {
        println!("{:<20}{:>20}{:>20}", "Category", "Foreign subtotal", "Domestic subtotal");
        for category in self.ordered_category_list() {
            let foreign_amount = match self.categories.get(category) {
                Some(foreign_amount) => foreign_amount,
                None => {
                    eprint!("Tried to read a category that does not exist from memory. Exiting.");
                    exit(1);
                }
            };
            println!("{:<20}{:>20.2}{:>20.2}", category, foreign_amount, foreign_amount * self.conversion_factor);
        }
    }
}

/// # Run main program logic
/// 
/// This function is run in `main.rs` as follows.
/// 
/// ```no_run
/// use forex_split::run;
/// 
/// fn main() {
///     run();
/// }
/// ```
pub fn run() -> () {
    let args = Args::new();
    let mut category_list = CategoryList::new(args);
    category_list.input_loop();
    category_list.print_split_table();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::collections::HashMap;

    #[test]
    fn category_list_new_works() -> Result<(), String> {
        let test_category_list = CategoryList::new(Args { domestic_total: Some(10f64), foreign_total: Some(20f64) });
        let _empty_hash_map: HashMap<String, f64> = HashMap::new();

        assert_eq!(test_category_list.conversion_factor, 0.5f64);
        assert_eq!(test_category_list.remaining_foreign_total, 20f64);
        assert_eq!(test_category_list.categories, _empty_hash_map);
        
        Ok(())
    }

    #[test]
    fn category_list_assign_to_category_works() -> Result<(), String> {
        let mut test_category_list = CategoryList::new(Args { domestic_total: Some(10f64), foreign_total: Some(20f64) });
        let mut _test_hash_map: HashMap<String, f64> = HashMap::new();
        _test_hash_map.insert("Foo".to_string(), 5f64);
        test_category_list.assign_to_category("Foo".to_string(), 5f64);

        assert_eq!(test_category_list.conversion_factor, 0.5f64);
        assert_eq!(test_category_list.remaining_foreign_total, 20f64);
        assert_eq!(test_category_list.categories, _test_hash_map);
        
        Ok(())
    }
}