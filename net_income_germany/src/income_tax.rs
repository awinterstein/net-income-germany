use crate::config::{IncomeTaxConfig, SolidaryAdditionConfig, TaxRange};

impl TaxRange {
    /// Calculate the range from the upper and lower limit.
    pub fn range(&self) -> u32 {
        self.upper_limit - self.lower_limit
    }
}

pub fn calculate(config: &IncomeTaxConfig, taxable_income: u32, together: bool) -> u32 {
    let tax = calculate_income_tax(&config, taxable_income, together);
    let tax_solidarity =
        calculate_solidarity_addition(tax, together, &config.solidary_addition_config);

    return tax + tax_solidarity;
}

fn deduct_tax_for_one_range(income: u32, tax_range: &TaxRange) -> f32 {
    // income so small, that this tax range does not apply
    if income <= tax_range.lower_limit {
        return 0.0;
    }

    // remove the lower limit from the income (as everything below is taxed in lower ranges)
    // and make sure that not more than the current tax range of the income is considered
    let taxed_income = (income - tax_range.lower_limit).min(tax_range.range());

    let income_range = tax_range.range() as f32;
    let taxed_income = taxed_income as f32;

    let rate_diff = tax_range.rate_max - tax_range.rate_min;
    let effective_rate_diff = taxed_income / income_range * rate_diff;

    let effective_rate = tax_range.rate_min + effective_rate_diff / 2.0;

    return taxed_income * effective_rate;
}

fn calculate_income_tax(config: &IncomeTaxConfig, income: u32, together: bool) -> u32 {
    let mut tax_sum = 0.0;

    // for married couples the taxes are calculated based on half of the combined income
    let income = if together { income / 2 } else { income };

    for tax_range in &config.tax_ranges {
        let tax = deduct_tax_for_one_range(income, tax_range);

        tax_sum = tax_sum + tax;
    }

    if together {
        // the tax value needs to be doubled again after calculating with half for married couples
        return tax_sum as u32 * 2;
    } else {
        return tax_sum as u32;
    }
}

fn calculate_solidarity_addition(
    tax: u32,
    together: bool,
    solidarity_addition_config: &SolidaryAdditionConfig,
) -> u32 {
    let tax_exemption_level = if together {
        solidarity_addition_config.exemption_level * 2
    } else {
        solidarity_addition_config.exemption_level
    };

    if tax < tax_exemption_level {
        return 0;
    }

    let max_solidarity_addition =
        (tax - tax_exemption_level) as f32 * solidarity_addition_config.max_percentage;
    let solidarity_addition = tax as f32 * solidarity_addition_config.rate;

    return solidarity_addition.min(max_solidarity_addition) as u32;
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
    fn test_tax_calculation_2024() {
        // the test data is based on the income tax calculator of the
        // German ministry of finances (https://www.bmf-steuerrechner.de)
        let test_data = vec![
            Data { i: 11791, o: 0 },
            Data { i: 11792, o: 1 },
            Data { i: 17008, o: 991 },
            Data { i: 18000, o: 1231 },
            Data { i: 46231, o: 9544 },
            Data { i: 66760, o: 17402 },
            Data {
                i: 277825,
                o: 111882,
            },
        ];

        test_tax_calculation(&test_data, 2024, false);
    }

    #[test]
    fn test_tax_calculation_married_2024() {
        // the test data is based on the income tax calculator of the
        // German ministry of finances (https://www.bmf-steuerrechner.de)
        let test_data = vec![
            Data { i: 23583, o: 0 },
            Data { i: 23584, o: 2 },
            Data { i: 50000, o: 6046 },
            Data { i: 66760, o: 10804 },
            Data {
                i: 277825,
                o: 100659,
            },
            Data {
                i: 555650,
                o: 223765,
            },
        ];

        test_tax_calculation(&test_data, 2024, true);
    }

    fn test_tax_calculation(test_data: &Vec<Data>, year: u32, together: bool) {
        let config = create_config(year).unwrap();

        for data in test_data {
            let result = calculate(&config.income_tax, data.i, together);
            assert_eq!(result, data.o);
        }
    }

    #[test]
    fn test_with_maximum_input_value() {
        let config = crate::config::Config::default();

        let result = calculate(&config.income_tax, std::u32::MAX, false);
        assert!(result > 2000000000); // check that there won't be some overflow that leads to a small result value
    }
}
