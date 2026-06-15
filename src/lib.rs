use clap::Parser;
use std::io;
use std::io::Write;

/// Flush to stdout. If it is not possible, print an error message.
macro_rules! flush {
    () => {
        io::stdout()
            .flush()
            .expect("Unable to write terminal output.")
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

    /// Return the domestic total from the command line arguments, if one exists. Otherwise ask the user to provide a domestic total.
    pub fn domestic_total(&self) -> f64 {
        match self.domestic_total {
            Some(total) => total,
            None => {
                print!("Please enter domestic total: ");
                flush!();
                input_float()
            },
        }
    }

    /// Return the foreign total from the command line arguments, if one exists. Otherwise ask the user to provide a foreign total.
    pub fn foreign_total(&self) -> f64 {
        match self.foreign_total {
            Some(total) => total,
            None => {
                print!("Please enter foreign total: ");
                flush!();
                input_float()
            },
        }
    }
}

/// Get terminal input and parse it to an f64. If no string that can be parsed to an f64 is provided, ask the user to provide a new string, until one that can be parsed to and f64 is provided. Prints an error message if unable to write get input from the terminal.
fn input_float() -> f64 {
    let parsed_input = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read terminal input.");
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