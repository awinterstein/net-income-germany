//! Calculates the net income from a given yearly gross income regarding German social security and
//! income tax laws.
//!
//! It can handle the calculations of the social security taxes for employed and for self-employed
//! income and can also take tax splitting for married couples into account.
//!
//! The following taxes and social security fees are considered for the calculation of the net
//! income:
//! - health insurance (Gesetzliche Krankenversicherung)
//! - nursing care insurance (Pflegeversicherung)
//! - unemployment insurance (Arbeitslosenversicherung)
//! - income tax (Einkommenssteuer)
//! - solidarity surcharge (Solidaritätszuschlag)
//!
//! # Example
//! ```
//! # fn main() -> Result<(), &'static str> {
//! // set the necessary input data values
//! let tax_data = net_income_germany::TaxData {
//!     income: 80000, // the gross income of one year
//!     expenses: 5300, // the tax-deductible expenses of one year
//!     fixed_retirement: Some(800), // an optional fixed monthly retirement rate (otherwise percentage applies)
//!     self_employed: false, // whether social security taxes should be calculated for a self-employed person
//!     married: false, // whether tax splitting due to marriage should apply
//! };
//!
//! // create the default configuration for a specific year (2024 and 2025 are supported)
//! let config = net_income_germany::config::create(2025)?;
//!
//! // do the tax calculation based on the input data values
//! let tax_result = net_income_germany::calculate(&config, &tax_data)?;
//!
//! // access the results (in this example just the resulting net income)
//! println!("Net income: {}", tax_result.net_income);
//!
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration Value Adaptions
//!
//! The configuration applies the additional fee to the nursing care insurance that is obligatory
//! for childless people over 23 years old. For the additional health insurance fee, it applies the
//! fee of Techniker Krankenkasse. In case that you want to change any of those to other values, you
//! can do it as follows:
//!
//! ```
//! # fn main() -> Result<(), &'static str> {
//! let mut config = net_income_germany::config::create(2025)?;
//! config.health_insurance.premium_additional = 0.0025; // change the additional health insurance fee [0,1]
//! config.health_insurance.premium_nursing_additional = 0.002; // change the additional nursing insurance fee [0,1]
//!
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

pub mod config;
mod income_tax;
mod social_security;

/// Input data struct for the tax calculation.
#[derive(Clone)]
pub struct TaxData {
    /// The gross or net income of one year (depending on whether calculate or calculate_reverse is called).
    pub income: u32,

    /// The expenses of one year that will be deducted from the gross income, before calculating the income taxes.
    pub expenses: u32,

    /// Optional value of a fixed monthly retirement insurance rate. If this is set, then this rate is used for every
    /// month. Otherwise, the retirement insurance rate is calculated by a percentage of the income.
    pub fixed_retirement: Option<u32>,

    /// Whether the calculations should be done for a self-employed person.
    pub self_employed: bool,

    /// Whether the income should be split for two people according to tax law.
    pub married: bool,
}

/// Result struct of the tax calculation.
pub struct TaxResult {
    /// The gross income before deducting social security taxes and income taxes.
    pub gross_income: i32,

    /// The net income after deducting social security taxes and income taxes.
    pub net_income: i32,

    /// The social security taxes that were deducted from the gross income.
    pub social_security_taxes: u32,

    /// The income taxes that were deducted from the gross income.
    pub income_taxes: u32,
}

impl TaxResult {
    /// Returns how much of the gross income was spent on social security and income taxes.
    pub fn get_tax_ratio(&self) -> f32 {
        let taxes = (self.social_security_taxes + self.income_taxes) as f32;
        return taxes / (self.net_income as f32 + taxes);
    }
}

/// Calculates social security taxes and income taxes based on the given income.
///
/// Returns the remaining net income and the calculated social security taxes and income taxes.
pub fn calculate(config: &config::Config, tax_data: &TaxData) -> Result<TaxResult, &'static str> {
    if tax_data.expenses < tax_data.income
        && tax_data.income - tax_data.expenses > std::i32::MAX as u32
    {
        return Err("Input values are too large to fit for the signed output.");
    }

    // calculate the social security taxes
    let social_security = social_security::calculate(
        &config.health_insurance,
        &config.retirement_insurance,
        &config.unemployment_insurance,
        &tax_data,
    )?;

    // reduce income by social security taxes and calculate income taxes on this
    let deductions = social_security + tax_data.expenses;
    let taxable_income = match deductions < tax_data.income {
        true => tax_data.income - deductions,
        false => 0,
    };
    let taxes = income_tax::calculate(&config.income_tax, taxable_income, tax_data.married);

    // store the results in the result struct
    let tax_result = TaxResult {
        gross_income: tax_data.income as i32,
        net_income: (tax_data.income as i64
            - tax_data.expenses as i64
            - social_security as i64
            - taxes as i64) as i32,
        social_security_taxes: social_security as u32,
        income_taxes: taxes,
    };

    return Ok(tax_result);
}

/// Calculates social security taxes and income taxes and from that the gross income based on the given net income.
///
/// This is the reverse calculation of the normal tax calculation, which would calculate the taxes and the net income
/// from the gross income.
///
/// Returns the remaining net income and the calculated social security taxes and income taxes.
pub fn calculate_reverse(
    config: &config::Config,
    tax_data: &TaxData,
) -> Result<TaxResult, &'static str> {
    let mut estimation = tax_data.income as f32 * 1.5; // first rough estimation of the gross income

    loop {
        // use given tax data (configuration) input, but replace the income
        // value with the estimated gross income
        let mut estimated_tax_data = tax_data.clone();
        estimated_tax_data.income = estimation as u32;
        let estimated_tax_data = estimated_tax_data;

        // calculate net income from the estimated gross income value
        let tax_result = calculate(config, &estimated_tax_data)?;

        // check how close the estimation of the gross income was by comparing
        // the calculated net income to the target net income value
        let estimation_difference = tax_result.net_income - tax_data.income as i32;
        estimation = estimation as f32 * (1.0 - estimation_difference as f32 / estimation as f32);

        // loop until the estimation of the gross income lead to the expected net income
        if estimation_difference == 0 {
            return Ok(tax_result);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate, calculate_reverse};
    use approx::abs_diff_eq;

    #[test]
    fn test_negative_net_income_employed() {
        let config = crate::config::Config::default();

        let tax_data = crate::TaxData {
            income: 0,
            expenses: 1500,
            fixed_retirement: None,
            self_employed: false,
            married: false,
        };

        let result = calculate(&config, &tax_data).unwrap();

        // no social security to be paid for employed person
        assert_eq!(result.social_security_taxes, 0);

        // net income is then just the negative expenses (no taxes)
        assert_eq!(
            result.net_income,
            tax_data.income as i32 - tax_data.expenses as i32
        );
    }

    #[test]
    fn test_negative_net_income_self_employed() {
        let config = crate::config::create(2025).unwrap();

        let tax_data = crate::TaxData {
            income: 0,
            expenses: 1500,
            fixed_retirement: None,
            self_employed: true,
            married: false,
        };

        let result = calculate(&config, &tax_data).unwrap();

        // minimum social security need to be paid for self-employed person
        assert_eq!(result.social_security_taxes, 3093);

        // net income is then just the negative expenses (no taxes)
        assert_eq!(
            result.net_income,
            tax_data.income as i32 - tax_data.expenses as i32 - result.social_security_taxes as i32
        );
    }

    #[test]
    fn test_reverse_tax_calculation() {
        let config = crate::config::Config::default();

        let tax_data_gross = crate::TaxData {
            income: 43000,
            expenses: 1500,
            fixed_retirement: None,
            self_employed: false,
            married: false,
        };

        // calculate net income from the given gross income
        let net_income = calculate(&config, &tax_data_gross).unwrap();

        // use the resulting net income then as input for the reverse tax calculation
        let mut tax_data_net = tax_data_gross.clone();
        tax_data_net.income = net_income.net_income as u32;

        // do the reverse calculation and expect that the same gross income is calculated again
        let gross_income = calculate_reverse(&config, &tax_data_net).unwrap();
        assert!(abs_diff_eq!(
            tax_data_gross.income as i32,
            gross_income.gross_income,
            epsilon = 1 // the gross income can vary a bit due to rounding up of the net income
        ));
    }
}
