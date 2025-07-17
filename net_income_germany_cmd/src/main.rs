//! Calculates and displays taxes and social insurances for a given income.
//!
//! The minimal command line call needs just the gross income of the year:
//! ```
//! $ net-income-germany-cmd --income 80000
//! ```

use clap::Parser;
use std::process;

/// Command line arguments of the application.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Annual income before taxes, social security and tax-deductible expenses (or net income in case of --reverse)
    #[arg(short, long)]
    income: u32,

    /// Tax-deductible expenses
    #[arg(short, long, default_value_t = 0)]
    expenses: u32,

    /// Fixed retirement rate (percentage will be calculated if not set)
    #[arg(short, long)]
    fixed_retirement: Option<u32>,

    /// Calculate social security and income taxes for a self-employed person
    #[arg(short, long)]
    self_employed: bool,

    /// Calculate with tax splitting for a married couple
    #[arg(short, long)]
    married: bool,

    /// For which year the taxes should be calculated
    #[arg(short, long, default_value_t = 2025)]
    year: u32,

    /// When set, the income is interpreted as net income and the gross income will be calculated from it
    #[arg(short, long)]
    reverse: bool,
}

/// Parses command line arguments, calls the net-income-germany crate then for
/// calculation of the taxes and social security premiums and prints the result
/// to the standard output.
fn main() {
    let args = Args::parse();

    let tax_data = net_income_germany::TaxData {
        income: args.income,
        expenses: args.expenses,
        fixed_retirement: args.fixed_retirement,
        self_employed: args.self_employed,
        married: args.married,
    };

    // create the tax configuration for the given year
    let config: net_income_germany::config::Config = net_income_germany::config::create(args.year)
        .unwrap_or_else(|err| {
            eprintln!("Failed to calculate the taxes: {err}");
            process::exit(1);
        });

    // Calculate the taxes with the configuration and the given tax data. This
    // can be either gross income to net income or net income to gross income
    // (reverse).
    let tax_result = match args.reverse {
        false => net_income_germany::calculate(&config, &tax_data),
        true => net_income_germany::calculate_reverse(&config, &tax_data),
    }
    .unwrap_or_else(|err| {
        eprintln!("Failed to calculate the taxes: {err}");
        process::exit(1);
    });

    println!(
        "Gross income: {}, net income: {}, social security taxes: {}, income taxes: {}, net ratio: {}",
        tax_result.gross_income,
        tax_result.net_income,
        tax_result.social_security_taxes,
        tax_result.income_taxes,
        1.0 - tax_result.get_tax_ratio()
    )
}
