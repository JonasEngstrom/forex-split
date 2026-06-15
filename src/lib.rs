use clap::Parser;
use std::io;
use std::io::Write;

/// Split a receipt in one currency into categories for bookkeeping in another currency.
#[derive(Parser)]
pub struct Args {
    /// Total in the currency from your bank statement, i.e. probably the currency domestic to your home country.
    domestic_total: Option<f64>,
    /// Total in the currency on the receipt from the purchase, i.e. the, to you, foreign currency from the country you are traveling in.
    foreign_total: Option<f64>,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn domestic_total(&self) -> f64 {
        match self.domestic_total {
            Some(total) => total,
            None => {
                print!("Please enter domestic total: ");
                io::stdout()
                    .flush()
                    .expect("Unable to write terminal output.");
                input_float()
            },
        }
    }

    pub fn foreign_total(&self) -> f64 {
        match self.foreign_total {
            Some(total) => total,
            None => {
                print!("Please enter foreign total: ");
                io::stdout()
                    .flush()
                    .expect("Unable to write terminal output.");
                input_float()
            },
        }
    }
}

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
                    io::stdout()
                        .flush()
                        .expect("Unable to write terminal outpu.");
                    continue;
                },
            }
    };
    parsed_input
}