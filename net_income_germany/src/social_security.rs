use crate::TaxData;
use crate::config::{
    HealthInsuranceConfig, RetirementInsuranceConfig, UnemploymentInsuranceConfig,
};

/// Calculate the social security payment from the given health and retirement insurance configuration and the tax data (yearly income).
pub fn calculate(
    health_insurance_config: &HealthInsuranceConfig,
    retirement_insurance_config: &RetirementInsuranceConfig,
    unemployment_insurance_config: &UnemploymentInsuranceConfig,
    tax_data: &TaxData,
) -> Result<u32, &'static str> {
    // for self-employed persons there is a minimum income that needs to be
    // used for the health insurance calculations in case that the actual
    // income is lower
    let income_for_health_insurance = match tax_data.self_employed {
        true => {
            let min_income_year = health_insurance_config.min_income * 12.0;
            tax_data.income.max(min_income_year as u32)
        }
        false => tax_data.income,
    };

    // calculate health insurance based on the given gross income (limited by the maximum configured income value)
    let health_insurance = calculate_social_insurance(
        income_for_health_insurance,
        calculate_health_insurance_premium(health_insurance_config, tax_data),
        health_insurance_config.max_income,
    );

    // calculate retirement insurance either from a given fixed value or as percentage from income
    let retirement_insurance = match tax_data.fixed_retirement {
        Some(fixed_retirement) => (fixed_retirement * 12) as f32,
        None => calculate_social_insurance(
            tax_data.income,
            calculate_retirement_insurance_premium(retirement_insurance_config, tax_data),
            retirement_insurance_config.max_income,
        ),
    };

    let unemployment_insurance = match tax_data.self_employed {
        true => 0.0,
        false => calculate_social_insurance(
            tax_data.income,
            unemployment_insurance_config.premium / 2.0,
            unemployment_insurance_config.max_income,
        ),
    };

    return Ok((health_insurance + retirement_insurance + unemployment_insurance) as u32);
}

/// Calculate the social security payment (for one insurance) based on the given yearly income and premium percentage.
///
/// The premium is limited by the maximum monthly income value to be considered for the calculation.
fn calculate_social_insurance(
    yearly_income: u32, // the yearly income on which the social security payment is calculated
    premium_percentage: f32, // how much of the income needs to be payed for the insurance
    max_monthly_value: f32, // the maximum monthly income that is considered for the premium (monthly upper income limit)
) -> f32 {
    let effective_income = (yearly_income as f32).min(max_monthly_value * 12.0);
    return effective_income * premium_percentage;
}

fn calculate_health_insurance_premium(
    health_insurance_config: &HealthInsuranceConfig,
    tax_data: &TaxData,
) -> f32 {
    if tax_data.self_employed {
        return health_insurance_config.premium_general_reduced
            + health_insurance_config.premium_additional
            + health_insurance_config.premium_nursing
            + health_insurance_config.premium_nursing_additional;
    } else {
        return (health_insurance_config.premium_general
            + health_insurance_config.premium_additional
            + health_insurance_config.premium_nursing)
            / 2.0
            + health_insurance_config.premium_nursing_additional;
    }
}

fn calculate_retirement_insurance_premium(
    retirement_insurance_config: &RetirementInsuranceConfig,
    tax_data: &TaxData,
) -> f32 {
    if tax_data.self_employed {
        return retirement_insurance_config.premium;
    } else {
        // the employer is paying half of the premium for an employee
        return retirement_insurance_config.premium / 2.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::create as create_config;

    struct Data {
        i: u32,
        o: u32,
    }

    #[test]
    fn test_social_security_calculation_2024() {
        let test_data = vec![
            Data { i: 12000, o: 2496 },
            Data { i: 25132, o: 5227 },
            Data { i: 62100, o: 12916 },
            Data { i: 90600, o: 15937 },
            Data { i: 99999, o: 15937 },
        ];

        test_social_security(&test_data, 2024, false, None);
    }

    #[test]
    fn test_social_security_calculation_self_employed_2025() {
        let test_data = vec![
            Data { i: 12000, o: 3093 },
            Data { i: 25128, o: 5188 },
            Data { i: 62100, o: 12823 },
            Data { i: 66150, o: 13659 },
            Data { i: 99999, o: 13659 },
        ];

        test_social_security(&test_data, 2025, true, Some(0));
    }

    fn test_social_security(
        test_data: &Vec<Data>,
        year: u32,
        self_employed: bool,
        fixed_retirement: Option<u32>,
    ) {
        let config = create_config(year).unwrap();

        for data in test_data {
            let tax_data = TaxData {
                income: data.i,
                expenses: 0,
                fixed_retirement: fixed_retirement,
                self_employed: self_employed,
                married: false,
            };

            let result = calculate(
                &config.health_insurance,
                &config.retirement_insurance,
                &config.unemployment_insurance,
                &tax_data,
            )
            .unwrap();
            assert_eq!(result, data.o);
        }
    }

    #[test]
    fn test_with_maximum_input_value() {
        let config = crate::config::Config::default();

        let tax_data = TaxData {
            income: u32::MAX,
            expenses: 0,
            fixed_retirement: None,
            self_employed: false,
            married: false,
        };

        let result = calculate(
            &config.health_insurance,
            &config.retirement_insurance,
            &config.unemployment_insurance,
            &tax_data,
        )
        .unwrap();
        assert_eq!(result, 17466);
    }
}
